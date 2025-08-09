use cliclack::{confirm, input};

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{Step, StepResult},
};

#[derive(Default)]
pub struct Breaking;

impl Step for Breaking {
    fn run(&mut self, state: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        let is_breaking = confirm("Is this a breaking change?")
            .initial_value(false)
            .interact()?;
        state.commit.set_is_breaking_change(Some(is_breaking));
        Ok(())
    }
}

#[derive(Default)]
pub struct Message {
    execute: bool,
}

impl Step for Message {
    fn before_run(
        &mut self,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.execute = state.commit.is_breaking_change.unwrap_or_default();
        Ok(())
    }

    fn run(&mut self, state: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        if !self.execute {
            return Ok(());
        }

        let breaking_change_msg: String = input("Expand the breaking change description")
            .required(false)
            .interact()?;

        let breaking_change_msg = (!breaking_change_msg.is_empty()).then_some(breaking_change_msg);

        state
            .commit
            .set_breaking_change_message(breaking_change_msg);
        Ok(())
    }
}
