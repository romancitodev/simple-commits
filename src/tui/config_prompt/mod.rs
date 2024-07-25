use std::io::Stderr;

use promptuity::{Error, Promptuity};

use crate::config::SimpleCommitsConfig;

pub fn init(
    prompt: &mut Promptuity<Stderr>,
    config: &mut SimpleCommitsConfig,
) -> Result<(), Error> {
    prompt.with_intro("Simple Commit").begin()?;

    prompt.step("Setting up configuration files")?;

    prompt.log("")?;
    prompt.info("Succesfully created.")?;
    prompt.log(format!("Path: {:?}", config.config))?;
    prompt.log("")?;

    prompt
        .with_outro(concat!(
            "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
            "\u{2764}  Thanks for use this tool!",
        ))
        .finish()?;

    Ok(())
}
