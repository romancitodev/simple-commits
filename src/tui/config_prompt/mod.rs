use crate::config::cli::SimpleCommitsConfig;
use cliclack::{
    intro,
    log::{info, step},
    outro,
};

pub fn init(
    SimpleCommitsConfig { config, .. }: &mut SimpleCommitsConfig,
) -> Result<(), std::io::Error> {
    intro("Simple Commit")?;

    step("Setting up configuration files")?;

    info("")?;
    info("Succesfully created.")?;
    info(format!("Path: {config:?}"))?;
    info("")?;

    outro(concat!(
        "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
        "\u{2764}  Thanks for use this tool!",
    ))?;

    Ok(())
}
