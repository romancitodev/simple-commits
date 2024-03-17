use inquire::Text;

use crate::{
    config::SimpleCommitsConfig,
    tui::{helpers::valid_length, Step, StepError, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State, _: &mut SimpleCommitsConfig) -> StepResult {
        let msg = Text::new("Commit message:")
            .with_validator(valid_length)
            .prompt();
        match msg {
            Ok(msg) if !msg.is_empty() => {
                state.msg = msg;
                Ok(())
            }
            Ok(_) => Err(StepError::NoMsg),
            Err(_) => Err(StepError::InvalidMsg),
        }
    }
}
