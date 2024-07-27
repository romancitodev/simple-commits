use log::{debug, error};
use promptuity::prompts::SelectOption;

use crate::{
    config::SimpleCommitsConfig,
    tui::{
        widgets::{Autocomplete, AutocompletePriority},
        Step, StepResult,
    },
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut promptuity::Promptuity<std::io::Stderr>,
        state: &mut crate::tui::State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let scopes = config.scopes.clone().unwrap_or_default();
        let mapped_scopes = scopes
            .scopes()
            .iter()
            .map(|scope| {
                SelectOption::new(scope.name(), scope.name().to_owned())
                    .with_hint(scope.description().clone().unwrap_or_default())
            })
            .collect::<Vec<_>>();
        let scope = p.prompt(&mut Autocomplete::new(
            "Select an scope",
            false,
            AutocompletePriority::Label,
            mapped_scopes,
        ))?;

        let scope = (!scope.is_empty()).then_some(scope);
        state.commit.set_scope(scope.clone());

        if let Some(scope) = scope {
            if let Some(scopes) = &mut config.scopes {
                if !scopes.exists(&scope) {
                    debug!(target: "steps::scope", "This shit works");
                    scopes.add_scope(scope.clone());
                    if let Err(err) = config.update() {
                        error!(target: "step::scope", "This shit aint work! {}", err);
                    }
                }
            }
        }

        Ok(())
    }
}
