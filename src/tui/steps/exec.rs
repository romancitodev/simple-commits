use std::io::Stderr;

use promptuity::prompts::Confirm;
use promptuity::Promptuity;

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
        _: &mut Promptuity<Stderr>,
        state: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config
            .git
            .as_ref()
            .map(|cfg| cfg.skip_preview)
            .unwrap_or(false);

        let commit = state.commit.clone().build()?;

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

    fn run(
        &mut self,
        p: &mut Promptuity<Stderr>,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        if self.skip {
            let (head, tail) = self.cmd.split_first().unwrap();
            self.action = Action::Commit(head.clone(), tail.to_vec());
            return Ok(());
        }

        let commit = state.commit.clone().build()?;

        let execute =
            p.prompt(Confirm::new("Do you want to execute this command?").with_default(true))?;
        if !execute {
            p.step("Commit preview")?;
            p.log(commit.0)?;
            p.log(BLANK_CHARACTER)?;
        } else {
            let (head, tail) = self.cmd.split_first().unwrap();
            self.action = Action::Commit(head.clone(), tail.to_vec());
        };

        Ok(())
    }

    fn after_run(
        &mut self,
        _: &mut Promptuity<Stderr>,
        _: &mut crate::tui::AppData,
        _config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.action.execute_action();
        Ok(())
    }
}
