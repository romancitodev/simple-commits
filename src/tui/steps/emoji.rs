use crate::{errors::AppError, gen::EMOJIS, reimpl::Pipeline};
use cliclack::select;
use log::info;

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

    let emojis_mapped = EMOJIS.map(|d| (d.emoji, format!("{} {}", d.emoji, d.description), d.name));

    let emoji = select("Select an emoji (optional)")
        .items(&emojis_mapped)
        .max_rows(8)
        .filter_mode()
        .interact()?;

    let emoji = (!emoji.is_empty() && emoji != "❌").then_some(emoji.to_owned());
    pipeline.state.commit.set_emoji(emoji.clone());

    info!(target: "tui::steps::emoji", "selected emoji: {emoji:?}");
    Ok(())
}
