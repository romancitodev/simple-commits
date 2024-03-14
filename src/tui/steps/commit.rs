use inquire::Select;

use crate::tui::{helpers::format_commits, structs::COMMIT_TYPES, Step, StepError, StepResult};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State) -> StepResult {
        let commit = Select::new("Select a commit:", COMMIT_TYPES.to_vec())
            .with_formatter(&format_commits)
            .prompt();

        state._type = commit
            .map(|c| c.label.to_string())
            .map_err(|_| StepError::InvalidMsg)?;
        Ok(())
    }
}
