use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{structs::COMMIT_TYPES, style},
};
use log::info;
use nobubbles::inline::select;

/// Step 1: Commit Type Definition
///
/// Prompts the user to select a commit type from a predefined list.
/// Updates the pipeline state with the selected commit type.
pub fn select_commit_type(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let mut commits = select(style::subtitle("Select a word"))
    .items(COMMIT_TYPES.map(|c| format!("{} {}", c.emoji, c.label)));

  for (idx, commit) in COMMIT_TYPES.iter().enumerate() {
    commits = commits.note(idx, commit.hint);
  }

  let commit = COMMIT_TYPES[commits.strict().ask()?];

  pipeline
    .state
    .commit
    .set_type(Some(commit.label.to_owned()));
  info!(target: "tui::steps::commit", "selected type: {commit}");

  Ok(())
}
