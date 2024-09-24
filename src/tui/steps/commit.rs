use cliclack::select;

use crate::{
    config::cli::SimpleCommitsConfig,
    tui::{structs::COMMIT_TYPES, Step, StepResult},
};

#[derive(Default)]
pub struct Definition;

impl Step for Definition {
    fn run(&mut self, state: &mut crate::tui::AppData, _: &mut SimpleCommitsConfig) -> StepResult {
        let mapped_commit =
            COMMIT_TYPES.map(|c| (c.label, format!("{} {}", c.emoji, c.label), c.hint));

        let commit = select("Select a word")
            .items(&mapped_commit)
            .filter_mode()
            .interact()?;

        state.commit.set_type(Some(commit.to_owned()));
        Ok(())
    }
}
