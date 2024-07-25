use promptuity::prompts::SelectOption;

use crate::config::SimpleCommitsConfig;
use crate::gen::EMOJIS;
use crate::tui::widgets::Autocomplete;
use crate::tui::{Step, StepResult};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut promptuity::Promptuity<std::io::Stderr>,
        state: &mut crate::tui::State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        if config.git.as_ref().is_some_and(|cfg| cfg.skip_emojis) {
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
            emojis_mapped,
            false,
        ))?;
        state.emoji = Some(emoji);
        Ok(())
    }
}
