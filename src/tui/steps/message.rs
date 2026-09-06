use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{helpers::valid_length, style},
};
use log::info;
use nobubbles::inline::input;

/// Step 6: Title Input
///
/// Prompts the user to enter a commit title/message.
/// If a message was provided via CLI, it will be used instead.
/// The title must be at least 5 characters long.
pub fn input_title(pipeline: &mut Pipeline) -> Result<(), AppError> {
  // Use title from CLI if provided
  if let Some(title) = &pipeline.config.message {
    pipeline.state.commit.set_title(Some(title.clone()));
    info!(target: "tui::steps::title", "using title from CLI: {title}");
    return Ok(());
  }

  let msg: String = input(style::subtitle("Enter a brief title of the commit"))
    .validate(|x: &str| valid_length(x, 5, "The commit must have at least 5 characters"))
    .ask()?;

  pipeline.state.commit.set_title(Some(msg.clone()));
  info!(target: "tui::steps::title", "entered title: {msg}");
  Ok(())
}
