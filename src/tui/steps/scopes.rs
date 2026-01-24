use crate::{errors::AppError, reimpl::Pipeline, tui::structs::InnerScope};
use cliclack::select;
use log::{debug, error, info};

/// Step 2: Scope Selection
///
/// Prompts the user to select a scope from available scopes in the config.
/// If a new scope is entered, it will be added to the config.
pub fn select_scope(pipeline: &mut Pipeline) -> Result<(), AppError> {
    let mut scopes = pipeline.config.scopes.clone().unwrap_or_default();
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
                scope.description().map_or(String::new(), Clone::clone),
            )
        })
        .collect::<Vec<_>>();

    let scope = select("Select a scope")
        .items(&mapped_scopes)
        .initial_value("none")
        .interact()?;

    let scope = (!scope.is_empty() && scope != "none").then_some(scope.to_owned());
    pipeline.state.commit.set_scope(scope.clone());

    // Add new scope to config if it doesn't exist
    let Some(scope) = scope else {
        info!(target: "tui::steps::scope", "no scope selected");
        return Ok(());
    };

    if pipeline
        .config
        .scopes
        .as_mut()
        .is_some_and(|s| !s.exists(&scope))
    {
        debug!(target: "steps::scope", "adding new scope: {scope}");
        scopes.add_scope(scope.clone());
        pipeline
            .config
            .update()
            .inspect_err(|err| {
                error!(target: "step::scope", "error updating the scopes: {err}");
            })
            .unwrap();
    }

    info!(target: "tui::steps::scope", "selected scope: {scope}");
    Ok(())
}
