use inquire::Confirm;

use crate::git::commit::{ColoredCommit, Commit};
use crate::{
    config::SimpleCommitsConfig,
    tui::{Step, StepResult},
};

#[derive(Clone, Debug)]
pub enum ExecType {
    Message(String),
    Command(String, Vec<String>),
}

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(&self, state: &mut crate::tui::State, config: &mut SimpleCommitsConfig) -> StepResult {
        if let Some(git) = config.git.as_ref() {
            let mut execute = true;

            let commit: Commit = state.clone().into();
            let commit = git
                .commit_template
                .as_ref()
                .map(|msg| {
                    msg.iter()
                        .map(|m| m.replace("{{message}}", &format!("{commit}")))
                        .collect::<Vec<String>>()
                })
                .unwrap_or(vec![
                    "git".to_string(),
                    "commit".to_string(),
                    "-m".to_string(),
                    format!("{commit}"),
                ]);

            let cmd = commit.first().expect("The commit template cannot be empty");
            if !git.skip_preview {
                execute = Confirm::new(&format!("Command to run: {}", commit.join(" ")))
                    .with_default(true)
                    .with_help_message("Do you want run these command?")
                    .prompt()
                    .is_ok_and(|c| c);
            }

            if execute {
                state.exec_type = Some(ExecType::Command(cmd.clone(), (&commit[1..]).to_vec()));
                return Ok(());
            }
        }

        let commit: ColoredCommit = state.clone().into();
        state.exec_type = Some(ExecType::Message(commit.to_string()));

        Ok(())
    }
}
