use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, style},
};
use log::info;
use nobubbles::inline::{confirm, password, task};

/// Step 9: Execute/Preview Commit
pub fn execute_commit(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let commit = pipeline.state.commit.clone().build().unwrap();

  let template = pipeline
    .config
    .git
    .as_ref()
    .and_then(|cfg| cfg.commit_template.clone());

  let skip_preview = pipeline
    .config
    .git
    .as_ref()
    .is_some_and(|cfg| cfg.skip_preview);

  let execute = skip_preview
    || confirm(style::subtitle("Do you want to execute this command?"))
      .initial(true)
      .ask()?;

  if !execute {
    nobubbles::inline::log::step("Commit preview");
    nobubbles::inline::log::block(&style::preview_card(&commit.0));
    info!(target: "tui::steps::execute", "commit preview shown");
    return Ok(());
  }

  // an arbitrary user command, not necessarily even `git commit`, so it stays shell-out
  if let Some(template) = template {
    let command = template
      .iter()
      .map(|msg| msg.replace("{{message}}", &commit.0))
      .collect::<Vec<_>>();

    let status = task(style::subtitle("Committing"), move |report| {
      git::run(&command, report)
    })??;

    if status.success() {
      nobubbles::inline::log::success("commit created");
    } else {
      nobubbles::inline::log::error(format!("git exited with {status}"));
    }

    info!(target: "tui::steps::execute", "commit executed (status: {status})");
    return Ok(());
  }

  let needs_passphrase = git::signing_enabled()?
    && !task(style::subtitle("Checking GPG"), |report| {
      report.say("waking up gpg-agent");
      let cached = git::passphrase_cached();
      report.say("gpg-agent ok");
      cached
    })??;
  let mut passphrase = needs_passphrase
    .then(|| password(style::subtitle("GPG passphrase")).ask())
    .transpose()?;

  const MAX_ATTEMPTS: u8 = 3;
  for attempt in 1..=MAX_ATTEMPTS {
    let message = commit.0.clone();
    let pass = passphrase.clone();
    match task(style::subtitle("Committing"), move |report| {
      git::commit(&message, pass.as_deref(), report)
    })? {
      Ok(()) => {
        nobubbles::inline::log::success("commit created");
        info!(target: "tui::steps::execute", "commit executed");
        break;
      }
      Err(AppError::BadPassphrase) if attempt < MAX_ATTEMPTS => {
        nobubbles::inline::log::warn("wrong passphrase, try again");
        passphrase = Some(password(style::subtitle("GPG passphrase")).ask()?);
      }
      Err(err) => {
        nobubbles::inline::log::error(err.to_string());
        info!(target: "tui::steps::execute", "commit failed: {err}");
        break;
      }
    }
  }

  Ok(())
}
