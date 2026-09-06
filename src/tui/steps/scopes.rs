use crate::{
  errors::AppError,
  reimpl::Pipeline,
  tui::{git, style},
};
use log::{debug, error, info};
use nobubbles::inline::autocomplete;

/// Step 2: Scope Selection
///
/// Prompts the user to select a scope from available scopes in the config, or type a new
/// one to add it. Leaving it empty (or typing "none") means no scope.
pub fn select_scope(pipeline: &mut Pipeline) -> Result<(), AppError> {
  let scopes = pipeline.config.scopes.clone().unwrap_or_default();
  let names: Vec<&str> = scopes.scopes().iter().map(|s| s.name()).collect();

  // The autocomplete field takes only a `'static` placeholder, so a branch match (borrowed
  // from a freshly-read branch name) can't be preloaded into it — surfaced as a log line
  // instead, for the user to type themselves.
  if let Some(scope) = git::current_branch()
    .ok()
    .flatten()
    .and_then(|branch| git::scope_from_branch(&branch, &names).map(str::to_owned))
  {
    nobubbles::inline::log::info(format!("branch suggests scope '{scope}'"));
  }

  let mut picker = autocomplete(style::subtitle("Select a scope"))
    .items(scopes.scopes().iter().map(|scope| scope.name().to_owned()))
    .placeholder("none");

  for (idx, scope) in scopes.scopes().iter().enumerate() {
    if let Some(description) = scope.description() {
      picker = picker.note(idx, description.clone());
    }
  }

  let answer = picker.ask()?;
  let answer = answer.trim();

  let scope = (!answer.is_empty() && answer != "none").then(|| answer.to_owned());
  pipeline.state.commit.set_scope(scope.clone());

  // Add new scope to config if it doesn't exist
  let Some(scope) = scope else {
    info!(target: "tui::steps::scope", "no scope selected");
    return Ok(());
  };

  if let Some(config_scopes) = pipeline.config.scopes.as_mut()
    && !config_scopes.exists(&scope)
  {
    debug!(target: "steps::scope", "adding new scope: {scope}");
    config_scopes.add_scope(scope.clone());
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
