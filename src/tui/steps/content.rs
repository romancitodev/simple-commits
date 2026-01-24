use crate::{errors::AppError, reimpl::Pipeline};
use cliclack::input;
use log::info;

/// Step 8: Footer Input
///
/// Prompts the user to enter optional footer notes for the commit.
/// This can be used for referencing issues, co-authors, or other metadata.
pub fn input_footer(pipeline: &mut Pipeline) -> Result<(), AppError> {
    let footer: String = input("Write some footer notes")
        .multiline()
        .required(false)
        .interact()?;

    let footer = (!footer.is_empty()).then_some(footer.clone());

    pipeline.state.commit.set_footer(footer.clone());

    info!(
        target: "tui::steps::footer",
        "footer: {footer:?}");

    Ok(())
}
