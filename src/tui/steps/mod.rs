use crate::{config::cli::SimpleCommitsConfig, errors::AppError, gen_steps};
use cliclack::{intro, outro};
use colored::Colorize;

pub mod body;
pub mod breaking_change;
pub mod commit;
pub mod content;
pub mod emoji;
pub mod exec;
pub mod scopes;
use log::{error, info};

use super::AppData;

pub fn init(config: &mut SimpleCommitsConfig) -> Result<(), AppError> {
    let mut state = AppData::default();
    let mut steps = gen_steps![
        commit::Definition,
        scopes::Scope,
        breaking_change::Breaking,
        breaking_change::Message,
        emoji::Emoji,
        content::Title,
        content::Body,
        content::Footer,
        exec::Execute
    ];

    intro("Simple Commit")?;

    for step in steps.iter_mut() {
        let _before = step.before_run(&mut state, config);
        let res = step.run(&mut state, config);
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            error!(target: "tui::steps", "{}", msg.bright_red());
            return Err(AppError::Step(msg));
        }
        let _after = step.after_run(&mut state, config);
        info!(target: "tui::steps", "steps: {state:#?}");
    }

    outro(concat!(
        "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
        "\u{2764}  Thanks for use this tool!",
    ))?;

    Ok(())
}
