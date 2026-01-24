use crate::{errors::AppError, reimpl::Pipeline};
use cliclack::confirm;
use log::info;

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

    if skip_preview {
        let (head, tail) = command.split_first().unwrap();
        let _ = std::process::Command::new(head)
            .args(tail)
            .spawn()
            .expect("The child failed for some reason")
            .wait();

        info!(target: "tui::steps::execute", "commit executed without preview");
    } else {
        let execute = confirm("Do you want to execute this command?")
            .initial_value(true)
            .interact()?;

        if execute {
            let (head, tail) = command.split_first().unwrap();
            let _ = std::process::Command::new(head)
                .args(tail)
                .spawn()
                .expect("The child failed for some reason")
                .wait();

            info!(target: "tui::steps::execute", "commit executed");
        } else {
            cliclack::log::step("Commit preview")?;
            cliclack::log::info(commit.0)?;
            cliclack::log::info("")?;

            info!(target: "tui::steps::execute", "commit preview shown");
        }
    }

    Ok(())
}
