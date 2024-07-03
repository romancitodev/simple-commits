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
        let commit: Commit = state.clone().into();
        let mut command = vec![
            "git".to_string(),
            "commit".to_string(),
            "-m".to_string(),
            format!("{commit}"),
        ];
        let cmd = command.first().expect("unreachable!").clone();
        if let Some(git) = &config.git {
            command = git
                .commit_template
                .as_ref()
                .map(|msg| {
                    msg.iter()
                        .map(|m| m.replace("{{message}}", &format!("{commit}")))
                        .collect::<Vec<String>>()
                })
                .unwrap_or(command);

            let cmd = command
                .first()
                .expect("The commit template cannot be empty");
            if git.preview_skip {
                state.exec_type = Some(ExecType::Command(cmd.clone(), (command[1..]).to_vec()));
                return Ok(());
            }
        }

        let execute = Confirm::new(&format!("Command to run: {}", commit))
            .with_default(true)
            .with_help_message("Do you want run these command?")
            .prompt()
            .is_ok_and(|c| c);

        if !execute {
            let commit: ColoredCommit = state.clone().into();
            state.exec_type = Some(ExecType::Message(commit.to_string()));
            return Ok(());
        }
        state.exec_type = Some(ExecType::Command(cmd.clone(), (command[1..]).to_vec()));

        Ok(())
    }
}
