use crate::{config::cli::AppConfig, errors::AppError, reimpl::Pipeline, tui::style};
use nobubbles::inline::{intro, outro};

// Step modules - each contains a function that works with the Pipeline
pub mod body;
pub mod breaking_change;
pub mod commit;
pub mod content;
pub mod emoji;
pub mod exec;
pub mod message;
pub mod scopes;
pub mod stage;

pub fn init(config: AppConfig) -> Result<(), AppError> {
  style::banner();
  let session = intro("Simple Commit")?;

  Pipeline::new(config)
    .then(stage::select_files)?
    .then(commit::select_commit_type)?
    .then(scopes::select_scope)?
    .if_then(
      breaking_change::ask_breaking_change,
      breaking_change::ask_breaking_message,
    )?
    .then(emoji::select_emoji)?
    .then(message::input_title)?
    .then(|args| {
      body::input_body(args);
      Ok(())
    })?
    .then(content::input_footer)?
    .then(exec::execute_commit)?;

  outro(session).with(concat!(
    "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
    "\u{2764}  Thanks for use this tool!",
  ));

  Ok(())
}
