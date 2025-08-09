use cliclack::confirm;
use cliclack::log::{info, step};

use crate::tui::helpers::BLANK_CHARACTER;
use crate::tui::Action;
use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{Step, StepResult},
};

#[derive(Default)]
pub struct Execute {
    skip: bool,
    action: Action,
    cmd: Vec<String>,
}

impl Step for Execute {
    fn before_run(
        &mut self,
        state: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config.git.as_ref().is_some_and(|cfg| cfg.skip_preview);

        let commit = state.commit.clone().build().unwrap();

        let command = {
            let base = ["git", "commit", "-m", &commit.0]
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<_>>();

            if let Some(cfg) = &config.git {
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

        self.cmd = command;

        Ok(())
    }

    fn run(&mut self, state: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        if self.skip {
            let (head, tail) = self.cmd.split_first().unwrap();
            self.action = Action::Commit(head.clone(), tail.to_vec());
            return Ok(());
        }

        let commit = state.commit.clone().build().unwrap();

        let execute = confirm("Do you want to execute this command?")
            .initial_value(true)
            .interact()?;
        if execute {
            let (head, tail) = self.cmd.split_first().unwrap();
            self.action = Action::Commit(head.clone(), tail.to_vec());
        } else {
            step("Commit preview")?;
            info(commit.0)?;
            info(BLANK_CHARACTER)?;
        }

        Ok(())
    }

    fn after_run(
        &mut self,
        _: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.action.execute_action();
        Ok(())
    }
}
