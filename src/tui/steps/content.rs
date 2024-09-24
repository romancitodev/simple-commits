use cliclack::input;

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{helpers::valid_length, AppData, Step, StepResult},
};

#[derive(Default)]
pub struct Title {
    skip: bool,
    title: Option<String>,
}

impl Step for Title {
    fn before_run(
        &mut self,
        _: &mut crate::tui::AppData,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult {
        self.skip = config.message.as_ref().is_some();
        self.title = config.message.clone();
        Ok(())
    }

    fn run(&mut self, _: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        if self.skip {
            return Ok(());
        }

        let msg = input("Enter a brief title of the commit")
            .required(true)
            .validate_interactively(|x: &String| {
                valid_length(x, 5, "The commit must have at least 5 characters")
            })
            .interact()?;

        self.title = Some(msg);
        Ok(())
    }

    fn after_run(
        &mut self,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        state.commit.set_title(self.title.clone());
        Ok(())
    }
}

#[derive(Default)]
pub struct Body;

impl Step for Body {
    fn before_run(
        &mut self,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn after_run(
        &mut self,
        _state: &mut AppData,
        _config: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        Ok(())
    }

    fn run(
        &mut self,
        app: &mut AppData,
        _: &mut crate::config::cli::SimpleCommitsConfig,
    ) -> StepResult {
        let body: String = input("Body").multiline().required(false).interact()?;
        app.commit.set_description(Some(body));
        Ok(())
    }
}

#[derive(Default)]
pub struct Footer;

impl Step for Footer {
    fn run(&mut self, state: &mut AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        let footer: String = input("Write some footer notes")
            .multiline()
            .required(false)
            .interact()?;

        let footer = (!footer.is_empty()).then_some(footer);

        state.commit.set_footer(footer);

        Ok(())
    }
}
