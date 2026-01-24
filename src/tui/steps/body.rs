use crate::{errors::AppError, reimpl::Pipeline};
use cliclack::input;
use log::info;

/// Step 7: Body/Description Input
///
/// Prompts the user to enter an optional multiline body/description for the commit.
/// This provides additional context beyond the title.
pub fn input_body(Pipeline { state, .. }: &mut Pipeline) -> Result<(), AppError> {
    let body: String = input("Body").multiline().required(false).interact()?;

    state.commit.set_description(Some(body.clone()));

    info!(
        target: "tui::steps::body",
        "body length: {} chars",
        body.len()
    );

    Ok(())
}
