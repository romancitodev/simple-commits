use crate::{
    errors::AppError,
    reimpl::Pipeline,
    tui::{structs::InnerScope, style},
};
use log::{debug, error, info};
use nobubbles::inline::select;

/// Step 2: Scope Selection
///
/// Prompts the user to select a scope from available scopes in the config.
/// If a new scope is entered, it will be added to the config.
pub fn select_scope(pipeline: &mut Pipeline) -> Result<(), AppError> {
    let mut scopes = pipeline.config.scopes.clone().unwrap_or_default();
    scopes
        .scopes
        .insert(0, InnerScope::new("none".to_owned(), None));

    let mut picker = select(style::subtitle("Select a scope"))
        .items(scopes.scopes().iter().map(|scope| scope.name().to_owned()));

    for (idx, scope) in scopes.scopes().iter().enumerate() {
        if let Some(description) = scope.description() {
            picker = picker.note(idx, description.clone());
        }
    }

    let idx = picker.strict().ask()?;
    let selected = scopes.scopes()[idx].name().to_owned();

    let scope = (idx != 0).then_some(selected);
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
