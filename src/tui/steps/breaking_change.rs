use crate::{errors::AppError, reimpl::Pipeline, tui::style};
use log::info;
use nobubbles::inline::{confirm, input};

/// Step 3: Breaking Change Confirmation
///
/// Prompts the user to confirm if this commit is a breaking change.
/// Updates the pipeline state with the breaking change flag.
pub fn ask_breaking_change(pipeline: &mut Pipeline) -> Result<bool, AppError> {
  let is_breaking = confirm(style::subtitle("Is this a breaking change?"))
    .initial(false)
    .ask()?;

  pipeline
    .state
    .commit
    .set_is_breaking_change(Some(is_breaking));

  info!(target: "tui::steps::breaking", "is breaking change: {is_breaking}");
  Ok(is_breaking)
}

/// Step 4: Breaking Change Message
///
/// If the commit is a breaking change, prompts the user for a detailed description.
/// This step is skipped if the commit is not a breaking change.
pub fn ask_breaking_message(pipeline: &mut Pipeline) -> Result<(), AppError> {
  // Skip if not a breaking change
  if !pipeline.state.commit.is_breaking_change.unwrap_or_default() {
    info!(target: "tui::steps::breaking_msg", "skipped - not a breaking change");
    return Ok(());
  }

  let breaking_change_msg: String =
    input(style::subtitle("Expand the breaking change description")).ask()?;

  let breaking_change_msg = (!breaking_change_msg.is_empty()).then_some(breaking_change_msg);

  pipeline
    .state
    .commit
    .set_breaking_change_message(breaking_change_msg.clone());

  info!(
      target: "tui::steps::breaking_msg",
      "breaking change message: {breaking_change_msg:?}"
  );
  Ok(())
}
