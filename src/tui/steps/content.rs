use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, style},
};
use log::info;
use nobubbles::inline::{input, select};

/// A commit rarely closes the issue it's on right away, most commits on a branch just work
/// towards it. `Refs` (a plain mention, no auto-close) is the safe default; `Closes` is an
/// explicit choice for the one commit that actually resolves it.
const ISSUE_TRAILERS: [&str; 3] = ["Refs", "Closes", "Skip"];

/// Step 8: Footer Input
///
/// Prompts the user to enter optional footer notes for the commit.
/// This can be used for referencing issues, co-authors, or other metadata.
pub fn input_footer(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let issue = git::current_branch()
    .ok()
    .flatten()
    .and_then(|branch| git::issue_from_branch(&branch));

  let trailer = issue
    .map(|id| {
      let choice = select(style::subtitle(format!(
        "Detected issue #{id} on this branch, reference it as?"
      )))
      .items(ISSUE_TRAILERS)
      .initial(0)
      .strict()
      .ask()?;

      Ok::<_, AppError>((choice < 2).then(|| format!("{}: #{id}", ISSUE_TRAILERS[choice])))
    })
    .transpose()?
    .flatten();

  let mut footer: String = input(style::subtitle("Write some footer notes"))
    .multiline()
    .placeholder("[skipped]")
    .ask()?;

  if let Some(trailer) = trailer
    && !footer.contains(&trailer)
  {
    if !footer.is_empty() {
      footer.push('\n');
    }
    footer.push_str(&trailer);
  }

  let footer = (!footer.is_empty()).then_some(footer.clone());

  pipeline.state.commit.set_footer(footer.clone());

  info!(
        target: "tui::steps::footer",
        "footer: {footer:?}");

  Ok(())
}
