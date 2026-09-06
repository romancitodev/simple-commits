//! Native git access via `git2`, plus the process-spawning still needed for a user's
//! `commit_template`, hooks, and `gpg`.

use crate::errors::AppError;
use git2::{Commit, Repository, Status, StatusOptions};
use log::info;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use tempfile::NamedTempFile;

/// Where progress lines go: `nobubbles::inline::Report::say` from a `task`, or `|_| {}` when
/// there's no live view to write into (a synchronous `validate`, say).
pub type Reporter<'a> = &'a dyn Fn(&str);

pub fn run(command: &[String], report: Reporter) -> std::io::Result<ExitStatus> {
  info!(target: "tui::git", "running command: {:?}", command);
  let (program, args) = command.split_first().expect("command can't be empty");

  let mut child = Command::new(program)
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  let (tx, rx) = channel();
  pipe(child.stdout.take(), &tx);
  pipe(child.stderr.take(), &tx);
  drop(tx); // last senders live in the reader threads, so `rx` ends when both pipes close

  for line in rx {
    report(&line);
  }

  child.wait()
}

fn pipe(stream: Option<impl Read + Send + 'static>, tx: &Sender<String>) {
  let Some(stream) = stream else { return };
  let tx = tx.clone();

  thread::spawn(move || {
    for line in BufReader::new(stream).lines().map_while(Result::ok) {
      let _ = tx.send(line);
    }
  });
}

fn open() -> Result<Repository, AppError> {
  Ok(Repository::discover(".")?)
}

pub fn signing_enabled() -> Result<bool, AppError> {
  let repo = open()?;
  Ok(repo.config()?.get_bool("commit.gpgsign").unwrap_or(false))
}

/// The current branch's short name (e.g. `feature/123-add-thing`), or `None` on a detached
/// HEAD or an unborn one (no commits yet).
pub fn current_branch() -> Result<Option<String>, AppError> {
  let repo = open()?;
  let Ok(head) = repo.head() else {
    return Ok(None);
  };
  if !head.is_branch() {
    return Ok(None);
  }
  Ok(head.shorthand().ok().map(str::to_owned))
}

const ISSUE_PREFIXES: [&str; 4] = ["gh", "issue", "ticket", "bug"];

/// Pulls a numeric issue id out of a branch name like `feature/123-add-thing`, `fix-142`,
/// `issue/57`, or `GH-9`. `None` when nothing in it looks like one.
pub fn issue_from_branch(branch: &str) -> Option<u32> {
  branch
    .split(|c: char| !c.is_alphanumeric())
    .filter(|token| !token.is_empty())
    .find_map(|token| {
      token.parse().ok().or_else(|| {
        let lower = token.to_lowercase();
        ISSUE_PREFIXES
          .iter()
          .find_map(|prefix| lower.strip_prefix(prefix)?.parse().ok())
      })
    })
}

/// The first branch segment (before `/`, `-`, or `_`) that matches one of `known_types`,
/// case-insensitively — e.g. `feat/123-thing` or `fix-142` both give a type at the front.
pub fn type_from_branch<'a>(branch: &str, known_types: &[&'a str]) -> Option<&'a str> {
  let first = branch.split(['/', '-', '_']).next()?;
  known_types
    .iter()
    .find(|t| t.eq_ignore_ascii_case(first))
    .copied()
}

fn gpg_identity(repo: &Repository) -> Result<(String, Option<String>), AppError> {
  let config = repo.config()?;
  let program = config
    .get_string("gpg.program")
    .unwrap_or_else(|_| "gpg".to_owned());
  let key = config.get_string("user.signingkey").ok();
  Ok((program, key))
}

// `--with-colons --with-keygrip` prints each key/subkey (`sec`/`ssb`) followed by an `fpr`
// then a `grp` line for it. A lowercase `s` in the capabilities field marks the (sub)key
// gpg actually signs with, which is the one gpg-agent caches the passphrase for.
fn signing_keygrip(program: &str, key: &str) -> Result<Option<String>, AppError> {
  let output = Command::new(program)
    .arg("--with-colons")
    .arg("--with-keygrip")
    .arg("--list-secret-keys")
    .arg(key)
    .output()?;

  let text = String::from_utf8_lossy(&output.stdout);
  let mut in_signing_key = false;

  for line in text.lines() {
    let mut fields = line.split(':');
    match fields.next() {
      Some("sec" | "ssb") => {
        in_signing_key = fields
          .nth(10)
          .is_some_and(|caps| caps.to_lowercase().contains('s'));
      }
      // The keygrip sits in the 10th field: `grp` then 8 empty fields, then it.
      Some("grp") if in_signing_key => return Ok(fields.nth(8).map(str::to_owned)),
      _ => {}
    }
  }

  Ok(None)
}

/// Whether `gpg-agent` already has this key's passphrase cached, checked directly through
/// `gpg-connect-agent` (`--pinentry-mode error` also answers this, but takes several seconds
/// on some setups instead of failing instantly like its name suggests).
pub fn passphrase_cached() -> Result<bool, AppError> {
  let repo = open()?;
  let (program, Some(key)) = gpg_identity(&repo)? else {
    return Ok(false);
  };
  let Some(keygrip) = signing_keygrip(&program, &key)? else {
    return Ok(false);
  };

  // A bare "gpg-connect-agent" resolves through `PATH`, which on a machine with more than one
  // GnuPG install can land on a different one than `gpg.program` — talking to a different
  // `gpg-agent`. Try the binary next to `program` first, then fall back to `PATH`.
  let keyinfo = format!("KEYINFO {keygrip}");
  let query = |gpg_connect_agent: &Path| {
    Command::new(gpg_connect_agent)
      .arg(&keyinfo)
      .arg("/bye")
      .output()
  };
  let sibling = Path::new(&program).with_file_name("gpg-connect-agent");

  let Ok(output) = query(&sibling).or_else(|_| query(Path::new("gpg-connect-agent"))) else {
    return Ok(false);
  };

  Ok(keyinfo_is_cached(&String::from_utf8_lossy(&output.stdout)))
}

/// A `KEYINFO` line looks like `S KEYINFO <keygrip> <type> <serialno> <idstr> <cached> ...`,
/// so `cached` is the fifth token, not the fourth (`idstr`, which is almost always `-`).
fn keyinfo_is_cached(response: &str) -> bool {
  response
    .lines()
    .find_map(|line| line.strip_prefix("S KEYINFO "))
    .and_then(|rest| rest.split_whitespace().nth(4))
    .is_some_and(|cached| cached == "1")
}

pub struct Change {
  pub path: String,
  pub note: &'static str,
}

/// The `git2` equivalent of `git status --porcelain`.
pub fn changes() -> Result<Vec<Change>, AppError> {
  let repo = open()?;

  let mut opts = StatusOptions::new();
  opts
    .include_untracked(true)
    .renames_head_to_index(true)
    .renames_index_to_workdir(true);

  let statuses = repo.statuses(Some(&mut opts))?;

  Ok(
    statuses
      .iter()
      .map(|entry| Change {
        path: entry.path().unwrap_or_default().to_owned(),
        note: describe(entry.status()),
      })
      .collect(),
  )
}

fn describe(status: Status) -> &'static str {
  if status.is_conflicted() {
    "conflict"
  } else if status.is_wt_new() {
    "untracked"
  } else if status.is_index_new() {
    "added"
  } else if status.is_wt_deleted() || status.is_index_deleted() {
    "deleted"
  } else if status.is_wt_renamed() || status.is_index_renamed() {
    "renamed"
  } else {
    "modified"
  }
}

// `Index::add_path` reads the file to stage it, so it can't handle a deleted one.
pub fn stage(paths: &[String]) -> Result<(), AppError> {
  let repo = open()?;
  let workdir = repo
    .workdir()
    .ok_or_else(|| AppError::Step("this repo has no working directory".to_owned()))?;
  let mut index = repo.index()?;

  for path in paths {
    if workdir.join(path).exists() {
      index.add_path(Path::new(path))?;
    } else {
      index.remove_path(Path::new(path))?;
    }
  }

  index.write()?;
  Ok(())
}

/// `passphrase` is only needed when [`signing_enabled`] is on and [`passphrase_cached`] came
/// back false. Pass `None` otherwise. Returns the new commit's id.
pub fn commit(
  message: &str,
  passphrase: Option<&str>,
  report: Reporter,
) -> Result<git2::Oid, AppError> {
  let repo = open()?;

  run_hook(
    &repo,
    "pre-commit",
    report,
    "pre-commit hook rejected the commit",
  )?;
  let message = run_commit_msg_hook(&repo, message, report)?;

  let mut index = repo.index()?;
  let tree = repo.find_tree(index.write_tree()?)?;
  let sig = repo.signature()?;
  let parent = match repo.head() {
    Ok(head) => Some(head.peel_to_commit()?),
    Err(_) => None,
  };
  // At most one parent, so a stack array stands in for the `Vec` a real merge would need.
  let parent = parent.as_ref().map(|c| [c]);
  let parents: &[&Commit] = parent.as_ref().map_or(&[], |one| one);

  let oid = if repo.config()?.get_bool("commit.gpgsign").unwrap_or(false) {
    sign_and_commit(&repo, &sig, &message, &tree, parents, passphrase, report)?
  } else {
    repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, parents)?
  };

  let _ = run_hook(&repo, "post-commit", report, ""); // best-effort

  Ok(oid)
}

fn hooks_dir(repo: &Repository) -> PathBuf {
  repo
    .config()
    .and_then(|cfg| cfg.get_path("core.hooksPath"))
    .unwrap_or_else(|_| repo.path().join("hooks"))
}

fn hook_path(repo: &Repository, name: &str) -> Option<PathBuf> {
  let path = hooks_dir(repo).join(name);
  is_executable(&path).then_some(path)
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
  use std::os::unix::fs::PermissionsExt;
  std::fs::metadata(path).is_ok_and(|meta| meta.permissions().mode() & 0o111 != 0)
}

// ponytail: no executable bit on Windows, and `Command::new` won't do the shebang launching
// git itself does for a hook script. Same ceiling as any non-git tool running hooks.
#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
  path.is_file()
}

/// `abort_message` turns a non-zero exit into an error; `""` means best-effort (`post-commit`).
/// No hook here takes extra arguments, so there's no `args` parameter to thread through.
fn run_hook(
  repo: &Repository,
  name: &str,
  report: Reporter,
  abort_message: &str,
) -> Result<(), AppError> {
  let Some(path) = hook_path(repo, name) else {
    return Ok(());
  };

  let status = run(&[path.to_string_lossy().into_owned()], report)?;
  if !status.success() && !abort_message.is_empty() {
    return Err(AppError::Step(abort_message.to_owned()));
  }

  Ok(())
}

fn run_commit_msg_hook(
  repo: &Repository,
  message: &str,
  report: Reporter,
) -> Result<String, AppError> {
  let Some(path) = hook_path(repo, "commit-msg") else {
    return Ok(message.to_owned());
  };

  let mut file = NamedTempFile::new()?;
  file.write_all(message.as_bytes())?;
  file.flush()?;

  let status = run(
    &[
      path.to_string_lossy().into_owned(),
      file.path().to_string_lossy().into_owned(),
    ],
    report,
  )?;
  if !status.success() {
    return Err(AppError::Step(
      "commit-msg hook rejected the commit".to_owned(),
    ));
  }

  Ok(std::fs::read_to_string(file.path())?)
}

/// `commit_signed`, unlike `commit`, doesn't move any ref, so this does it by hand afterward.
/// The commit content isn't secret and goes to `gpg` through a temp file. `passphrase` is
/// secret and goes through stdin (`--pinentry-mode loopback`), never touching disk; when it's
/// `None` (the [`passphrase_cached`] fast path), those flags are skipped entirely so `gpg`
/// talks to `gpg-agent` on its own and reuses the cached passphrase instead of being handed
/// an explicit empty one.
fn sign_and_commit(
  repo: &Repository,
  sig: &git2::Signature,
  message: &str,
  tree: &git2::Tree,
  parents: &[&Commit],
  passphrase: Option<&str>,
  report: Reporter,
) -> Result<git2::Oid, AppError> {
  report("signing commit");

  let buf = repo.commit_create_buffer(sig, sig, message, tree, parents)?;
  let content = std::str::from_utf8(&buf)
    .map_err(|_| AppError::Step("commit content wasn't valid utf-8".to_owned()))?;

  let mut content_file = NamedTempFile::new()?;
  content_file.write_all(content.as_bytes())?;
  content_file.flush()?;

  let (program, key) = gpg_identity(repo)?;

  let mut command = Command::new(&program);
  command.arg("--batch");
  if passphrase.is_some() {
    command
      .arg("--pinentry-mode")
      .arg("loopback")
      .arg("--passphrase-fd")
      .arg("0");
  }
  command
    .arg("--detach-sign")
    .arg("--armor")
    .arg("-o")
    .arg("-");
  if let Some(key) = &key {
    command.arg("--local-user").arg(key);
  }
  command.arg(content_file.path());

  let mut child = command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  if let Some(passphrase) = passphrase {
    writeln!(child.stdin.take().expect("stdin is piped"), "{passphrase}")?;
  }

  let output = child.wait_with_output()?;
  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.to_lowercase().contains("bad passphrase") {
      return Err(AppError::BadPassphrase);
    }
    // gpg often repeats the same complaint across several lines; the last non-empty one is
    // the most specific, and the only one worth showing.
    let summary = stderr.lines().rev().find(|line| !line.trim().is_empty());
    return Err(AppError::Step(format!(
      "gpg signing failed: {}",
      summary.unwrap_or("unknown error").trim()
    )));
  }

  let signature = String::from_utf8(output.stdout)
    .map_err(|_| AppError::Step("gpg produced a non-utf8 signature".to_owned()))?;

  let oid = repo.commit_signed(content, signature.trim_end(), None)?;

  let target = repo
    .find_reference("HEAD")?
    .symbolic_target()?
    .ok_or_else(|| AppError::Step("HEAD isn't a symbolic reference".to_owned()))?
    .to_owned();
  repo.reference(&target, oid, true, "commit (signed)")?;

  report("commit signed");
  Ok(oid)
}

#[cfg(test)]
mod tests {
  use super::*;

  // Needs this repo's real `user.signingkey` and a running gpg, so it's opt-in:
  // `cargo test -- --ignored gpg_cache`.
  #[test]
  #[ignore]
  fn gpg_cache_probe_is_fast_and_correct() {
    let repo = open().unwrap();
    let (program, key) = gpg_identity(&repo).unwrap();
    let key = key.expect("this repo has user.signingkey set");

    let grip = signing_keygrip(&program, &key).unwrap();
    assert!(grip.is_some(), "should resolve a signing-capable keygrip");

    let start = std::time::Instant::now();
    passphrase_cached().unwrap();
    assert!(
      start.elapsed().as_secs() < 2,
      "the cache check should be near-instant, not shell out to a real sign attempt"
    );
  }

  #[test]
  fn keyinfo_parses_the_cached_flag_from_the_right_field() {
    let not_cached = "S KEYINFO 73760AA1 D - - - P - - -\nOK\n";
    assert!(!keyinfo_is_cached(not_cached));

    let cached = "S KEYINFO 73760AA1 D - - 1 P - - -\nOK\n";
    assert!(
      keyinfo_is_cached(cached),
      "the 5th token is `cached`, not the 4th (`idstr`)"
    );
  }

  #[test]
  fn issue_and_type_are_pulled_from_branch_segments() {
    assert_eq!(issue_from_branch("feature/123-add-thing"), Some(123));
    assert_eq!(issue_from_branch("fix-142"), Some(142));
    assert_eq!(issue_from_branch("GH-9-cleanup"), Some(9));
    assert_eq!(issue_from_branch("issue57"), Some(57));
    assert_eq!(issue_from_branch("main"), None);

    let types = ["feat", "fix", "chore"];
    assert_eq!(type_from_branch("feat/123-thing", &types), Some("feat"));
    assert_eq!(type_from_branch("fix-142", &types), Some("fix"));
    assert_eq!(type_from_branch("random-branch", &types), None);
  }

  // No real gpg here on purpose: libgit2 never validates the signature string it's handed,
  // it just stores it, so this only needs to prove *our* plumbing (buffer -> commit_signed ->
  // moving HEAD's ref by hand, including the no-parent case) without depending on a real key,
  // a running gpg-agent, or whatever happens to be cached, on any platform.
  #[test]
  fn commit_signed_moves_the_ref_and_carries_the_header() {
    let dir = tempfile::tempdir().unwrap();
    let repo = Repository::init(dir.path()).unwrap();
    {
      let mut config = repo.config().unwrap();
      config.set_str("user.name", "Test User").unwrap();
      config.set_str("user.email", "test@example.com").unwrap();
    }
    std::fs::write(dir.path().join("a.txt"), "hello").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("a.txt")).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = repo.signature().unwrap();

    let buf = repo
      .commit_create_buffer(&sig, &sig, "test commit", &tree, &[])
      .unwrap();
    let content = std::str::from_utf8(&buf).unwrap();
    let fake_signature = "-----BEGIN PGP SIGNATURE-----\n\nnot real, just checking the plumbing\n-----END PGP SIGNATURE-----";

    let oid = repo.commit_signed(content, fake_signature, None).unwrap();
    let target = repo
      .find_reference("HEAD")
      .unwrap()
      .symbolic_target()
      .unwrap()
      .unwrap()
      .to_owned();
    repo
      .reference(&target, oid, true, "commit (signed)")
      .unwrap();

    let head = repo.head().unwrap().peel_to_commit().unwrap();
    assert_eq!(
      head.id(),
      oid,
      "HEAD should follow the ref onto a repo with no prior commits"
    );
    assert_eq!(head.message().unwrap(), "test commit");
    assert_eq!(
      std::str::from_utf8(&head.header_field_bytes("gpgsig").unwrap()).unwrap(),
      fake_signature
    );
  }
}
