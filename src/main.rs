mod commit;
mod config;
mod files;
mod options;
mod ui;

use colored::*;
use files::Settings;
use inquire::{error::InquireResult, Text};

use inquire::Select;
use options::{fetch_emojis, format_commits, format_emojis, Emoji, COMMIT_TYPES};

use crate::commit::Commit;

#[tokio::main]
async fn main() -> InquireResult<()> {
    config::install();

    let mut config = Settings::new().unwrap();

    let Ok(commit_type) = Select::new("\u{1F389} Select a type of commit:", COMMIT_TYPES.to_vec())
        .with_formatter(&format_commits)
        .prompt()
    else {
        let message = "❌ Required type for commit.".red();
        panic!("{}", message);
    };

    let scope = Text::new("Select the scope")
        .with_help_message("press ESC or type nothing to skip this.")
        .with_placeholder("None")
        .with_autocomplete(config.scope_autocomplete())
        .prompt_skippable();

    let scope = match scope {
        Ok(Some(scope)) => match &*scope {
            "" => None,
            scope => {
                if !config.exists_scope(scope) {
                    config.add_scope(scope);
                }
                Some(scope.to_string())
            }
        },
        Ok(None) => None,
        Err(_) => panic!("oh my god!"),
    };

    let emoji = Select::new("\u{1F36A} Select a type of commit:", fetch_emojis().await)
        .with_formatter(&format_emojis)
        .prompt_skippable();

    let emoji = match emoji {
        Ok(Some(c)) => Some(c.emoji()),
        Ok(None) => None,
        Err(e) => panic!("{}", e),
    };

    let title = Text::new("\u{1F4C3} Enter a title for the commit:")
        .prompt()
        .unwrap();

    println!("Your commit might be like:");
    println!(
        "{}",
        Commit::new(commit_type.label().to_string(), emoji, scope, title)
    );

    Ok(())
}
