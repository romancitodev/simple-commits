use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, structs::COMMIT_TYPES, style},
};
use log::info;
use nobubbles::inline::select;

/// Step 1: Commit Type Definition
///
/// Prompts the user to select a commit type from a predefined list.
/// Updates the pipeline state with the selected commit type.
pub fn select_commit_type(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let labels = COMMIT_TYPES.map(|c| c.label);
  let inferred = git::current_branch()
    .ok()
    .flatten()
    .and_then(|branch| git::type_from_branch(&branch, &labels))
    .and_then(|label| labels.iter().position(|l| *l == label));

  let mut commits = select(style::subtitle("Select a word"))
    .items(COMMIT_TYPES.map(|c| format!("{} {}", c.emoji, c.label)))
    .filter();

  if let Some(idx) = inferred {
    commits = commits.initial(idx);
  }

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
