//! Native git access via `git2`, plus the process-spawning still needed for a user's
//! `commit_template`, hooks, and `gpg`.

use crate::errors::AppError;
use git2::{Commit, Repository, Status, StatusOptions};
use log::info;
use nobubbles::inline::Report;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use tempfile::NamedTempFile;

pub fn run(command: &[String], report: &Report) -> std::io::Result<ExitStatus> {
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
    report.say(line);
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
  let lines: Vec<&str> = text.lines().collect();

  for (at, line) in lines.iter().enumerate() {
    let mut fields = line.split(':');
    let record = fields.next().unwrap_or_default();
    let can_sign = fields
      .nth(10)
      .is_some_and(|caps| caps.to_lowercase().contains('s'));

    if record != "sec" && record != "ssb" || !can_sign {
      continue;
    }

    let grip = lines[at + 1..]
      .iter()
      .take_while(|next| !next.starts_with("sec:") && !next.starts_with("ssb:"))
      .find_map(|next| next.strip_prefix("grp:"))
      .map(|rest| rest.trim_matches(':').to_owned());

    if grip.is_some() {
      return Ok(grip);
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

  let Ok(output) = Command::new("gpg-connect-agent")
    .arg(format!("KEYINFO {keygrip}"))
    .arg("/bye")
    .output()
  else {
    return Ok(false);
  };

  Ok(
    String::from_utf8_lossy(&output.stdout)
      .lines()
      .find_map(|line| line.strip_prefix("S KEYINFO "))
      .and_then(|rest| rest.split_whitespace().nth(3))
      .is_some_and(|cached| cached == "1"),
  )
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
/// back false. Pass `None` otherwise.
pub fn commit(message: &str, passphrase: Option<&str>, report: &Report) -> Result<(), AppError> {
  let repo = open()?;

  run_hook(&repo, "pre-commit", &[], report, "pre-commit hook rejected the commit")?;
  let message = run_commit_msg_hook(&repo, message, report)?;

  let mut index = repo.index()?;
  let tree = repo.find_tree(index.write_tree()?)?;
  let sig = repo.signature()?;
  let parent = match repo.head() {
    Ok(head) => Some(head.peel_to_commit()?),
    Err(_) => None,
  };
  let parents: Vec<&Commit> = parent.iter().collect();

  if repo.config()?.get_bool("commit.gpgsign").unwrap_or(false) {
    sign_and_commit(
      &repo,
      &sig,
      &message,
      &tree,
      &parents,
      passphrase.unwrap_or(""),
      report,
    )?;
  } else {
    repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &parents)?;
  }

  let _ = run_hook(&repo, "post-commit", &[], report, ""); // best-effort

  Ok(())
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
fn run_hook(
  repo: &Repository,
  name: &str,
  args: &[String],
  report: &Report,
  abort_message: &str,
) -> Result<(), AppError> {
  let Some(path) = hook_path(repo, name) else {
    return Ok(());
  };

  let mut command = vec![path.to_string_lossy().into_owned()];
  command.extend(args.iter().cloned());

  let status = run(&command, report)?;
  if !status.success() && !abort_message.is_empty() {
    return Err(AppError::Step(abort_message.to_owned()));
  }

  Ok(())
}

fn run_commit_msg_hook(
  repo: &Repository,
  message: &str,
  report: &Report,
) -> Result<String, AppError> {
  let Some(path) = hook_path(repo, "commit-msg") else {
    return Ok(message.to_owned());
  };

  let mut file = NamedTempFile::new()?;
  file.write_all(message.as_bytes())?;
  file.flush()?;
  let msg_path = file.path().to_string_lossy().into_owned();

  let status = run(
    &[path.to_string_lossy().into_owned(), msg_path.clone()],
    report,
  )?;
  if !status.success() {
    return Err(AppError::Step(
      "commit-msg hook rejected the commit".to_owned(),
    ));
  }

  Ok(std::fs::read_to_string(&msg_path)?)
}

/// `commit_signed`, unlike `commit`, doesn't move any ref, so this does it by hand afterward.
/// The commit content isn't secret and goes to `gpg` through a temp file; the passphrase is
/// secret and goes through stdin (`--pinentry-mode loopback`), never touching disk.
fn sign_and_commit(
  repo: &Repository,
  sig: &git2::Signature,
  message: &str,
  tree: &git2::Tree,
  parents: &[&Commit],
  passphrase: &str,
  report: &Report,
) -> Result<(), AppError> {
  report.say("signing commit");

  let buf = repo.commit_create_buffer(sig, sig, message, tree, parents)?;
  let content = std::str::from_utf8(&buf)
    .map_err(|_| AppError::Step("commit content wasn't valid utf-8".to_owned()))?;

  let mut content_file = NamedTempFile::new()?;
  content_file.write_all(content.as_bytes())?;
  content_file.flush()?;

  let (program, key) = gpg_identity(repo)?;

  let mut command = Command::new(&program);
  command
    .arg("--batch")
    .arg("--pinentry-mode")
    .arg("loopback")
    .arg("--passphrase-fd")
    .arg("0")
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

  writeln!(child.stdin.take().expect("stdin is piped"), "{passphrase}")?;

  let output = child.wait_with_output()?;
  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Step(format!("gpg signing failed: {stderr}")));
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

  report.say("commit signed");
  Ok(())
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

  // Exercises commit_create_buffer + the real gpg subprocess + commit_signed + the manual ref
  // move, against a disposable repo, using this machine's real signing key.
  // `cargo test -- --ignored signed_commit`.
  #[test]
  #[ignore]
  fn signed_commit_end_to_end() {
    if !passphrase_cached().unwrap() {
      eprintln!("skipping: gpg-agent's cache is cold, sign something for real first");
      return;
    }

    let dir = tempfile::tempdir().unwrap();
    let repo = Repository::init(dir.path()).unwrap();
    {
      let mut config = repo.config().unwrap();
      config.set_str("user.name", "Test User").unwrap();
      config.set_str("user.email", "test@example.com").unwrap();
      config.set_str("user.signingkey", "600119FFAF3321AF").unwrap();
    }
    std::fs::write(dir.path().join("a.txt"), "hello").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("a.txt")).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = repo.signature().unwrap();

    // cache is warm from other runs in this process, so any string does
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
      sign_and_commit_for_test(&repo, &sig, "test commit", &tree, &[], "irrelevant")
    }));

    let head = repo.head().unwrap().peel_to_commit().unwrap();
    assert_eq!(head.message().unwrap(), "test commit");
    assert!(
      head.header_field_bytes("gpgsig").is_ok(),
      "commit should carry a gpgsig header"
    );
    result.unwrap().unwrap();
  }

  fn sign_and_commit_for_test(
    repo: &Repository,
    sig: &git2::Signature,
    message: &str,
    tree: &git2::Tree,
    parents: &[&Commit],
    passphrase: &str,
  ) -> Result<(), AppError> {
    let buf = repo.commit_create_buffer(sig, sig, message, tree, parents)?;
    let content = std::str::from_utf8(&buf).unwrap();

    let mut content_file = NamedTempFile::new()?;
    content_file.write_all(content.as_bytes())?;
    content_file.flush()?;

    let (program, key) = gpg_identity(repo)?;
    let mut command = Command::new(&program);
    command
      .arg("--batch")
      .arg("--pinentry-mode")
      .arg("loopback")
      .arg("--passphrase-fd")
      .arg("0")
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
    writeln!(child.stdin.take().unwrap(), "{passphrase}")?;
    let output = child.wait_with_output()?;
    assert!(
      output.status.success(),
      "gpg failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    let signature = String::from_utf8(output.stdout).unwrap();
    let oid = repo.commit_signed(content, signature.trim_end(), None)?;

    let target = repo
      .find_reference("HEAD")?
      .symbolic_target()?
      .unwrap()
      .to_owned();
    repo.reference(&target, oid, true, "commit (signed)")?;
    Ok(())
  }
}
