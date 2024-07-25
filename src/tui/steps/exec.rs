use std::io::Stderr;

use promptuity::prompts::Confirm;
use promptuity::Promptuity;

use crate::git::commit::{ColoredCommit, Commit};
use crate::{
    config::SimpleCommitsConfig,
    tui::{Step, StepResult},
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Action {
    DryRun(String),
    Commit(String, Vec<String>),
}

impl Action {
    /// Returns the execute action of this [`Action`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn execute_action(&self) {
        match self {
            Self::DryRun(msg) => println!("{msg}"),
            Self::Commit(cmd, args) => {
                std::process::Command::new(cmd)
                    .args(&args[..])
                    .spawn()
                    .unwrap();
            }
        }
    }
}

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut Promptuity<Stderr>,
        state: &mut crate::tui::State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
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
            if git.skip_preview {
                state.exec_type = Some(Action::Commit(cmd.clone(), (command[1..]).to_vec()));
                return Ok(());
            }
        }

        let execute =
            p.prompt(Confirm::new("Do you want to execute this command?").with_default(true))?;
        if !execute {
            let commit: ColoredCommit = state.clone().into();
            p.step("Commit preview")?;
            p.log(commit.to_string())?;
        } else {
            state.exec_type = Some(Action::Commit(cmd.clone(), (command[1..]).to_vec()));

            state
                .exec_type
                .as_ref()
                .expect("At this point the exec type is filled")
                .execute_action();
        };

        Ok(())
    }
}
