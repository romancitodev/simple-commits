use cliclack::input;

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{helpers::valid_length, Step, StepResult},
};

#[derive(Default)]
pub struct Title {
    skip: bool,
    title: Option<String>,
}

impl Step for Title {
    fn before_run(
        &mut self,
        _: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config.message.as_ref().is_some();
        self.title = config.message.clone();
        Ok(())
    }

    fn run(&mut self, _: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        if self.skip {
            return Ok(());
        }

        let msg = input("Enter a brief title of the commit")
            .required(true)
            .validate_interactively(|x: &String| {
                valid_length(x, 5, "The commit must have at least 5 characters")
            })
            .interact()?;

        self.title = Some(msg);
        Ok(())
    }

    fn after_run(
        &mut self,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        state.commit.set_title(self.title.clone());
        Ok(())
    }
}
