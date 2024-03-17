use inquire::Select;

use crate::{
    config::FileConfig,
    tui::{Step, StepError, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State, config: &mut FileConfig) -> StepResult {
        let scopes = config.scopes.clone().unwrap_or_default();
        let scope =
            Select::new("Select a scope:", scopes.scopes().unwrap_or_default()).prompt_skippable();

        state.scope = scope.map_err(|_| StepError::NoCommit)?;

        let scope = state.scope.clone();

        if let Some(scope) = scope {
            if let Some(scopes) = &mut config.scopes {
                if !scopes.exists(&scope) {
                    scopes.add_scope(scope)
                }
            }
        }

        Ok(())
    }
}
