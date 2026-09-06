use crate::{errors::AppError, reimpl::Pipeline, tui::style};
use log::info;
use nobubbles::inline::input;

/// Step 8: Footer Input
///
/// Prompts the user to enter optional footer notes for the commit.
/// This can be used for referencing issues, co-authors, or other metadata.
pub fn input_footer(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let footer: String = input(style::subtitle("Write some footer notes"))
    .multiline()
    .ask()?;

  let footer = (!footer.is_empty()).then_some(footer.clone());

  pipeline.state.commit.set_footer(footer.clone());

  info!(
        target: "tui::steps::footer",
        "footer: {footer:?}");

  Ok(())
}
