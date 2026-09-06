use crate::{
    errors::AppError,
    reimpl::Pipeline,
    tui::{git, style},
};
use log::info;
use nobubbles::inline::{confirm, task};

/// Step 9: Execute/Preview Commit
///
/// Builds the final commit message and either executes it or shows a preview.
/// The behavior depends on the `skip_preview` configuration.
pub fn execute_commit(pipeline: &mut Pipeline) -> Result<(), AppError> {
    let commit = pipeline.state.commit.clone().build().unwrap();

    let command = {
        let base = ["git", "commit", "-m", &commit.0]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>();

        if let Some(cfg) = &pipeline.config.git {
            cfg.commit_template.as_ref().map_or_else(
                || base,
                |cfg| {
                    cfg.iter()
                        .map(|msg| msg.replace("{{message}}", &commit.0))
                        .collect::<Vec<_>>()
                },
            )
        } else {
            base
        }
    };

    let skip_preview = pipeline
        .config
        .git
        .as_ref()
        .is_some_and(|cfg| cfg.skip_preview);

    let execute = skip_preview
        || confirm(style::subtitle("Do you want to execute this command?"))
            .initial(true)
            .ask()?;

    if execute {
        let status = task(style::subtitle("Committing"), move |report| {
            git::run(&command, report)
        })??;

        if status.success() {
            nobubbles::inline::log::success("commit created");
        } else {
            nobubbles::inline::log::error(format!("git exited with {status}"));
        }

        info!(target: "tui::steps::execute", "commit executed (status: {status})");
    } else {
        nobubbles::inline::log::step("Commit preview");
        nobubbles::inline::log::block(&style::preview_card(&commit.0));

        info!(target: "tui::steps::execute", "commit preview shown");
    }

    Ok(())
}
