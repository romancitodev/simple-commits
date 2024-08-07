use std::io::Stderr;

use promptuity::{prompts::SelectOption, Promptuity};

use crate::tui::widgets::{Autocomplete, AutocompletePriority};
use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{structs::COMMIT_TYPES, Step, StepResult},
};

#[derive(Default)]
pub struct Definition;

impl Step for Definition {
    fn run(
        &mut self,
        p: &mut Promptuity<Stderr>,
        state: &mut crate::tui::AppData,
        _: &mut SimpleCommitsConfig,
    ) -> StepResult {
        let commit = p.prompt(&mut Autocomplete::new(
            "Select a type",
            true,
            AutocompletePriority::Label,
            COMMIT_TYPES
                .map(|c| SelectOption::new(c, c.label.to_owned()).with_hint(c.hint))
                .to_vec(),
        ));

        state.commit.set_type(Some(commit?));
        Ok(())
    }
}
