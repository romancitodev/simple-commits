use std::io::Stderr;

use super::State;
use crate::{config::cli::SimpleCommitsConfig, gen_steps};
use colored::Colorize;

pub mod commit;
pub mod emoji;
pub mod exec;
pub mod message;
pub mod scopes;
use log::{error, info};
use promptuity::{Error, Promptuity};

pub fn init(
    prompt: &mut Promptuity<Stderr>,
    config: &mut SimpleCommitsConfig,
) -> Result<(), Error> {
    let mut state = State::default();
    let steps = gen_steps![commit, scopes, emoji, message, exec];

    prompt.with_intro("Simple Commit").begin()?;

    for step in steps {
        let res = step.run(prompt, &mut state, config);
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            error!(target: "tui::steps", "{}", msg.bright_red());
            return Err(Error::Prompt(String::from("Error")));
        }
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
