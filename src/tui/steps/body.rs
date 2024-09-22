use cliclack::input;

use crate::tui::{AppData, Step, StepResult};

#[derive(Default)]
pub struct Body;

impl Step for Body {
    fn before_run(
        &mut self,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn after_run(
        &mut self,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn run(
        &mut self,
        app: &mut AppData,
        _: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        let body: String = input("Body").multiline().required(false).interact()?;
        app.commit.set_description(Some(body));
        Ok(())
    }
}
