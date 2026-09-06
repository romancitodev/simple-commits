use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, style},
};
use log::info;
use nobubbles::inline::input;

/// Step 8: Footer Input
///
/// Prompts the user to enter optional footer notes for the commit.
/// This can be used for referencing issues, co-authors, or other metadata.
pub fn input_footer(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let mut footer: String = input(style::subtitle("Write some footer notes"))
    .multiline()
    .ask()?;

  if let Some(issue) = git::current_branch()
    .ok()
    .flatten()
    .and_then(|branch| git::issue_from_branch(&branch))
    && !footer.contains(&format!("#{issue}"))
  {
    if !footer.is_empty() {
      footer.push('\n');
    }
    footer.push_str(&format!("Closes: #{issue}"));
    nobubbles::inline::log::info(format!("detected issue #{issue} from the branch name"));
  }

  let footer = (!footer.is_empty()).then_some(footer.clone());

  pipeline.state.commit.set_footer(footer.clone());

  info!(
        target: "tui::steps::footer",
        "footer: {footer:?}");

  Ok(())
}
