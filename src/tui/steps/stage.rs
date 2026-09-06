use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, style},
};
use log::info;
use nobubbles::inline::{multiselect, task};

/// Step 0: File Selection
///
/// Lists every changed path, tracked or not, and stages only the ones picked.
///
/// Unlike a `--dry-run`, this really stages the files, so ticking an untracked file stages it
/// for real, there's no separate "add this new file" step to remember.
pub fn select_files(_pipeline: &mut Pipeline) -> Result<(), AppError> {
  let changes = git::changes()?;

  if changes.is_empty() {
    nobubbles::inline::log::warn("nothing to commit, the working tree is clean");
    return Err(AppError::Step("nothing to commit".to_owned()));
  }

  let mut picker = multiselect(style::subtitle("What goes in the commit?"))
    .items(changes.iter().map(|change| change.path.clone()))
    .max_rows(10);

  for (idx, change) in changes.iter().enumerate() {
    picker = picker.note(idx, change.note);
  }

  let chosen = picker.ask()?;

  if chosen.is_empty() {
    nobubbles::inline::log::warn("nothing picked, nothing staged");
    return Err(AppError::Step("nothing picked".to_owned()));
  }

  let paths: Vec<String> = chosen.iter().map(|&at| changes[at].path.clone()).collect();

  let count = paths.len();
  let listing = paths.join(", ");

  if let Err(err) = task(style::subtitle("Staging"), move |report| {
    report.say(format!("git add -- {listing}"));
    git::stage(&paths)
  })? {
    nobubbles::inline::log::error(err.to_string());
    return Err(err);
  }

  nobubbles::inline::log::success(format!(
    "staged {count} file{}",
    if count == 1 { "" } else { "s" }
  ));

  info!(target: "tui::steps::stage", "staged {count} file(s)");
  Ok(())
}
