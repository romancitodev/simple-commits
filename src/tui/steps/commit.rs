use inquire::Select;

use crate::tui::{helpers::format_commits, structs::COMMIT_TYPES, Step};

#[derive(Default)]
pub struct CommitStep {
    _type: String,
}

impl Step for CommitStep {
    fn run(&self, state: &mut crate::tui::State) {
        let commit = Select::new("Select a commit:", COMMIT_TYPES.to_vec())
            .with_formatter(&format_commits)
            .prompt();

        if let Ok(commit) = commit {
            state._type = commit.label.to_string();
        } else {
            println!("Error...")
        }
    }
}
