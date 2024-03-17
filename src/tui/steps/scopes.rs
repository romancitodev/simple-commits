use inquire::Text;

use crate::{
    config::SimpleCommitsConfig,
    tui::{structs::ScopesAutoComplete, Step, StepError, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State, config: &mut SimpleCommitsConfig) -> StepResult {
        let scopes = config.scopes.clone().unwrap_or_default();
        let autocomplete: ScopesAutoComplete = scopes.clone().into();
        let scope = Text::new("Select a scope:")
            .with_placeholder("app")
            .with_autocomplete(autocomplete)
            .prompt_skippable();

        state.scope = scope.map_err(|_| StepError::NoCommit)?;

        let scope = state.scope.clone();

        if let Some(scope) = scope {
            if let Some(scopes) = &mut config.scopes {
                if !scopes.exists(&scope) {
                    scopes.add_scope(scope);
                    if let Err(err) = config.update() {
                        eprintln!("{err}");
                    }
                }
            }
        }

        Ok(())
    }
}
