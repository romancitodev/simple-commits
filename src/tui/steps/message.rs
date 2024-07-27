use promptuity::prompts::Input;

use crate::{
    config::SimpleCommitsConfig,
    tui::{helpers::valid_length, Step, StepResult},
};

#[derive(Default)]
pub struct _Step;

impl Step for _Step {
    fn run(
        &self,
        p: &mut promptuity::Promptuity<std::io::Stderr>,
        state: &mut crate::tui::State,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let msg = p.prompt(
            Input::new("Enter a brief title of the commit").with_validator(|text: &String| {
                valid_length(
                    text,
                    5,
                    "The commit message must have at least 5 characters",
                )
            }),
        )?;
        state.commit.set_title(Some(msg));
        Ok(())
    }
}
