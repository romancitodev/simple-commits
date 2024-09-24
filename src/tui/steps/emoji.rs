use cliclack::select;

use crate::config::cli::SimpleCommitsConfig;
use crate::gen::EMOJIS;
use crate::tui::{Step, StepResult};

#[derive(Default)]
pub struct Emoji {
    skip: bool,
}

impl Step for Emoji {
    fn before_run(
        &mut self,
        _: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config.git.as_ref().is_some_and(|cfg| cfg.skip_emojis);
        Ok(())
    }

    fn run(&mut self, state: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        if self.skip {
            return Ok(());
        }

        let emojis_mapped =
            EMOJIS.map(|d| (d.emoji, format!("{} {}", d.emoji, d.description), d.name));

        let emoji = select("Select an emoji (optional)")
            .items(&emojis_mapped)
            .filter_mode()
            .interact()?;

        let emoji = (!emoji.is_empty() && emoji != "❌").then_some(emoji.to_owned());
        state.commit.set_emoji(emoji);

        Ok(())
    }
}
