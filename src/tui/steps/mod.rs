use crate::{config::cli::SimpleCommitsConfig, gen_steps};
use colored::Colorize;

pub mod body;
pub mod commit;
pub mod emoji;
pub mod exec;
pub mod message;
pub mod scopes;
use log::{error, info};
use promptuity::Error;

use super::{AppData, Prompt};

pub fn init(prompt: &mut Prompt, config: &mut SimpleCommitsConfig) -> Result<(), Error> {
    let mut state = AppData::default();
    let mut steps = gen_steps![
        commit::Definition,
        scopes::Scope,
        emoji::Emoji,
        message::Title,
        body::Body,
        exec::Execute
    ];

    prompt.with_intro("Simple Commit").begin()?;

    for step in steps.iter_mut() {
        let _before = step.before_run(prompt, &mut state, config);
        let res = step.run(prompt, &mut state, config);
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            error!(target: "tui::steps", "{}", msg.bright_red());
            return Err(Error::Prompt(String::from("Error")));
        }
        let _after = step.after_run(prompt, &mut state, config);
        info!(target: "tui::steps", "steps: {state:#?}");
    }

    prompt
        .with_outro(concat!(
            "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
            "\u{2764}  Thanks for use this tool!",
        ))
        .finish()?;

    Ok(())
}
