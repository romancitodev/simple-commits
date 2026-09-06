use crate::{reimpl::Pipeline, tui::style};
use log::info;
use nobubbles::inline::input;

/// Step 7: Body/Description Input
///
/// Prompts the user to enter an optional multiline body/description for the commit.
/// This provides additional context beyond the title.
pub fn input_body(Pipeline { state, .. }: &mut Pipeline) {
  let body: String = input(style::subtitle("Body"))
    .multiline()
    .ask()
    .expect("Failed to read body input");

  state.commit.set_description(Some(body.clone()));

  info!(
      target: "tui::steps::body",
      "body length: {} chars",
      body.len()
  );
}
