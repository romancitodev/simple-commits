use crate::tui::{widgets::MultiInput, AppData, Prompt, Step, StepResult};

#[derive(Default)]
pub struct Body;

impl Step for Body {
    fn before_run(
        &mut self,
        _prompt: &mut Prompt,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn after_run(
        &mut self,
        _prompt: &mut Prompt,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn run(
        &mut self,
        p: &mut Prompt,
        _: &mut AppData,
        _: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        let _ = p.prompt(&mut MultiInput::new());
        Ok(())
    }
}
