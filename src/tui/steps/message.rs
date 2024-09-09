use promptuity::prompts::Input;

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{helpers::valid_length, Prompt, Step, StepResult},
};

#[derive(Default)]
pub struct Title {
    skip: bool,
    title: Option<String>,
}

impl Step for Title {
    fn before_run(
        &mut self,
        _: &mut Prompt,
        _: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config.message.as_ref().is_some();
        self.title = config.message.clone();
        Ok(())
    }

    fn run(
        &mut self,
        p: &mut Prompt,
        _: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        if self.skip {
            return Ok(());
        }

        let msg = p.prompt(
            Input::new("Enter a brief title of the commit").with_validator(|text: &String| {
                valid_length(
                    text,
                    5,
                    "The commit message must have at least 5 characters",
                )
            }),
        )?;
        self.title = Some(msg);
        Ok(())
    }

    fn after_run(
        &mut self,
        _: &mut Prompt,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        state.commit.set_title(self.title.clone());
        Ok(())
    }
}
