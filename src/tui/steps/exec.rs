use std::io::Stderr;

use promptuity::prompts::Confirm;
use promptuity::Promptuity;

use crate::tui::helpers::BLANK_CHARACTER;
use crate::tui::Action;
use crate::{
    config::SimpleCommitsConfig,
    tui::{Step, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut Promptuity<Stderr>,
        state: &mut crate::tui::State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let commit = state.commit.clone().build()?;
        let mut command = vec![
            "git".to_string(),
            "commit".to_string(),
            "-m".to_string(),
            format!("{0}", commit.0),
        ];
        let cmd = command.first().expect("unreachable!").clone();
        if let Some(git) = &config.git {
            command = git
                .commit_template
                .as_ref()
                .map(|msg| {
                    msg.iter()
                        .map(|m| m.replace("{{message}}", &commit.0.to_string()))
                        .collect::<Vec<String>>()
                })
                .unwrap_or(command);

            let cmd = command
                .first()
                .expect("The commit template cannot be empty");
            if git.skip_preview {
                state.action = Action::Commit(cmd.clone(), (command[1..]).to_vec());
                state.action.execute_action();
                return Ok(());
            }
        }

        let execute =
            p.prompt(Confirm::new("Do you want to execute this command?").with_default(true))?;
        if !execute {
            p.step("Commit preview")?;
            p.log(commit.0)?;
            p.log(BLANK_CHARACTER)?;
        } else {
            state.action = Action::Commit(cmd.clone(), (command[1..]).to_vec());

            state.action.execute_action();
        };

        Ok(())
    }
}
