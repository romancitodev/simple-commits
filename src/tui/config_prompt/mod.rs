use crate::config::cli::AppConfig;
use crate::errors::AppError;
// use nobubbles::inline::{
//     intro,
//     log::{info, step},
//     outro,
// };

use nobubbles::inline::log::*;
use nobubbles::inline::*;

pub fn init(AppConfig { config, .. }: AppConfig) -> Result<(), AppError> {
    let session = intro("Simple Commit")?;

    step("Setting up configuration files");

    info("");
    info("Succesfully created.");
    info(format!("Path: {}", config.display()));
    info("");

    outro(session).with(concat!(
        "In case of issues, please report it to https://github.com/romancitodev/simple-commits\n",
        "\u{2764}  Thanks for use this tool!",
    ));

    Ok(())
}
