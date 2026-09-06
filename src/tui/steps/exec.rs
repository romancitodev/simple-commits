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

  nobubbles::inline::log::step("Commit preview");
  nobubbles::inline::log::block(&style::preview_card(&commit.0));
  info!(target: "tui::steps::execute", "commit preview shown");

  let execute = confirm(style::subtitle("Do you want to execute this command?"))
    .initial(true)
    .ask()?;

  if !execute {
    return Ok(());
  }

  // an arbitrary user command, not necessarily even `git commit`, so it stays shell-out
  if let Some(template) = template {
    let command = template
      .iter()
      .map(|msg| msg.replace("{{message}}", &commit.0))
      .collect::<Vec<_>>();

    let status = task(style::subtitle("Committing"), move |report| {
      git::run(&command, &|line| report.say(line))
    })??;

    if status.success() {
      nobubbles::inline::log::success("commit created");
      nobubbles::inline::log::block(&style::success_card(&commit.0));
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

  if needs_passphrase {
    // The whole commit happens inside `validate`, on submit: a wrong passphrase refuses the
    // answer and the same prompt stays up asking again, instead of a fresh "GPG passphrase"
    // / "Committing" pair stacking up in the transcript for every attempt.
    let message = commit.0.clone();
    password(style::subtitle("GPG passphrase"))
      .invisible()
      .validate(move |pass| {
        git::commit(&message, Some(pass), &|_| {})
          .map(|_| ())
          .map_err(|err| match err {
            AppError::BadPassphrase => "wrong passphrase, try again".to_owned(),
            other => other.to_string(),
          })
      })
      .ask()?;

    nobubbles::inline::log::success("commit created");
    nobubbles::inline::log::block(&style::success_card(&commit.0));
    info!(target: "tui::steps::execute", "commit executed");
    return Ok(());
  }

  let message = commit.0.clone();
  match task(style::subtitle("Committing"), move |report| {
    git::commit(&message, None, &|line| report.say(line))
  })? {
    Ok(_) => {
      nobubbles::inline::log::success("commit created");
      nobubbles::inline::log::block(&style::success_card(&commit.0));
      info!(target: "tui::steps::execute", "commit executed");
    }
    Err(err) => {
      nobubbles::inline::log::error(err.to_string());
      info!(target: "tui::steps::execute", "commit failed: {err}");
    }
  }

  Ok(())
}
