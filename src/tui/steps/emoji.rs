use crate::{errors::AppError, gitmoji::EMOJIS, reimpl::Pipeline, tui::style};
use log::info;
use nobubbles::inline::select;

/// Step 5: Emoji Selection
///
/// Prompts the user to select an optional emoji for the commit.
/// This step can be skipped if configured in the settings.
pub fn select_emoji(pipeline: &mut Pipeline) -> Result<(), AppError> {
  // Skip emoji selection if configured
  if pipeline
    .config
    .git
    .as_ref()
    .is_some_and(|cfg| cfg.skip_emojis)
  {
    info!(target: "tui::steps::emoji", "skipped - emojis disabled in config");
    return Ok(());
  }

  let mut emojis = select(style::subtitle("Select an emoji (optional)"))
    .items(EMOJIS.map(|e| format!("{} {}", e.emoji, e.description)))
    .skip("[skipped]")
    .max_rows(8)
    .filter();

  for (idx, emoji) in EMOJIS.iter().enumerate() {
    if !emoji.name.is_empty() {
      emojis = emojis.note(idx, emoji.name);
    }
  }

  let emoji = emojis.ask()?.map(|idx| EMOJIS[idx].emoji.to_owned());
  pipeline.state.commit.set_emoji(emoji.clone());

  info!(target: "tui::steps::emoji", "selected emoji: {emoji:?}");
  Ok(())
}
