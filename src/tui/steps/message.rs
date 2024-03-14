use inquire::Text;

use crate::tui::{Step, StepError, StepResult};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State) -> StepResult {
        let msg = Text::new("Commit message:").prompt();
        state.msg = msg.map_err(|_| StepError::InvalidMsg)?;
        Ok(())
    }
}
