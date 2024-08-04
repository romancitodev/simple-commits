use promptuity::prompts::SelectOption;

use crate::config::cli::SimpleCommitsConfig;
use crate::gen::EMOJIS;
use crate::tui::widgets::{Autocomplete, AutocompletePriority};
use crate::tui::{Step, StepResult};

#[derive(Default)]
pub struct Emoji {
    skip: bool,
}

impl Step for Emoji {
    fn before_run(
        &mut self,
        _: &mut promptuity::Promptuity<std::io::Stderr>,
        _: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
    self.skip = config.git.as_ref().is_some_and(|cfg| cfg.skip_emojis);

        Ok(())
    }

    fn run(
        &mut self,
        p: &mut promptuity::Promptuity<std::io::Stderr>,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        if self.skip {
            return Ok(());
        }
        
        let emojis_mapped = EMOJIS
            .map(|emoji| {
                SelectOption::new(
                    format!("{} {}", emoji.emoji, emoji.description),
                    emoji.emoji.to_string(),
                )
                .with_hint(emoji.name)
            })
            .to_vec();
        let emoji = p.prompt(&mut Autocomplete::new(
            "Select an emoji (optional)",
            false,
            AutocompletePriority::Hint,
            emojis_mapped,
        ))?;

        let emoji = (!emoji.is_empty()).then_some(emoji);
        state.commit.set_emoji(emoji);

        Ok(())
    }
}
