use crate::{errors::AppError, reimpl::Pipeline, tui::structs::COMMIT_TYPES};
use cliclack::select;
use log::info;

/// Step 1: Commit Type Definition
///
/// Prompts the user to select a commit type from a predefined list.
/// Updates the pipeline state with the selected commit type.
pub fn select_commit_type(pipeline: &mut Pipeline) -> Result<(), AppError> {
    let mapped_commit = COMMIT_TYPES.map(|c| (c.label, format!("{} {}", c.emoji, c.label), c.hint));

    let commit = select("Select a word")
        .items(&mapped_commit)
        .filter_mode()
        .interact()?;

    pipeline.state.commit.set_type(Some(commit.to_owned()));
    info!(target: "tui::steps::commit", "selected type: {commit}");

    Ok(())
}
