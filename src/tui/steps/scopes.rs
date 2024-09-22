use cliclack::select;
use log::{debug, error};

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{structs::InnerScope, Step, StepResult},
};

#[derive(Default)]
pub struct Scope;

impl Step for Scope {
    fn run(
        &mut self,
        state: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let mut scopes = config.scopes.clone().unwrap_or_default();
        scopes
            .scopes
            .insert(0, InnerScope::new("none".to_owned(), None));

        let mapped_scopes = scopes
            .scopes()
            .iter()
            .map(|scope| {
                (
                    scope.name(),
                    scope.name(),
                    scope.description().clone().unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>();

        let scope = select("Select an scope")
            .filter_mode()
            .items(&mapped_scopes)
            .initial_value("none")
            .interact()?;

        let scope = (!scope.is_empty() && scope != "none").then_some(scope.to_owned());
        state.commit.set_scope(scope.clone());

        // FIX: Error on global path
        let Some(scope) = scope else { return Ok(()) };
        if config.scopes.as_mut().is_some_and(|s| !s.exists(&scope)) {
            debug!(target: "steps::scope", "adding scope");
            scopes.add_scope(scope);
            config
                .update()
                .inspect_err(|err| {
                    error!(target: "step::scope", "error updating the scopes: {}", err);
                })
                .unwrap();
        }

        Ok(())
    }
}
