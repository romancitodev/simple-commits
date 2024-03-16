use inquire::Select;

use crate::gen::EMOJIS;
use crate::tui::{helpers::format_emojis, Step, StepError, StepResult};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State) -> StepResult {
        let emoji = Select::new("Select an emoji (optional):", EMOJIS.to_vec())
            .with_formatter(&format_emojis)
            .prompt_skippable();

        state.emoji = emoji
            .map(|emoji| emoji.map(|e| e.emoji.to_string()))
            .map_err(|_| StepError::NoCommit)?;
        Ok(())
    }
}
