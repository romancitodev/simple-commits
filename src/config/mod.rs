use clap::Parser;
use directories::BaseDirs;
use merge2::Merge;
use serde::Deserialize;
use std::path::PathBuf;

use crate::tui::structs::Scopes;

mod handle_config;

#[derive(Deserialize, Parser, Merge)]
/// Cli config settings.
pub struct CliConfig {
    #[clap(long, help = "set custom path to load config.")]
    #[merge(skip)]
    #[serde(skip)]
    config: Option<PathBuf>,

    #[clap(long, conflicts_with = "local")]
    global: bool,

    #[clap(long, conflicts_with = "global")]
    local: bool,

    #[clap(flatten)]
    file: FileConfig,
}

/// File settings for customizing the bin.
#[derive(Deserialize, Parser, Merge)]
pub struct FileConfig {
    #[clap(flatten)]
    pub scopes: Option<Scopes>,
}

pub fn get_config() -> FileConfig {
    let mut args = CliConfig::parse();

    // check if the config path was provided.
    let config_path = if let Some(path) = args.config {
        path
    } else {
        let path = BaseDirs::new().unwrap().config_dir().join("sc");
        path.join("config.toml")
    };

    args.file
}
