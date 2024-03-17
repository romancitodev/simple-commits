use clap::Parser;
use directories::BaseDirs;
use merge2::Merge;
use serde::Deserialize;
use std::env::current_dir;
use std::path::PathBuf;

use crate::tui::structs::Scopes;

mod handle_config;

#[inline]
pub fn swap_option<T>(left: &mut Option<T>, right: &mut Option<T>) {
    if left.is_none() || right.is_some() {
        core::mem::swap(left, right);
    }
}

#[derive(Parser)]
/// Cli config settings.
pub struct CliConfig {
    #[clap(long, help = "set custom path to load config.")]
    config: Option<PathBuf>,

    #[clap(flatten)]
    sc_config: SimpleCommitsConfig,
}

/// File settings for customizing the bin.
#[derive(Default, Deserialize, Parser, Merge)]
pub struct SimpleCommitsConfig {
    #[clap(flatten)]
    #[merge(strategy = swap_option)]
    pub scopes: Option<Scopes>,
}

pub fn get_config() -> SimpleCommitsConfig {
    let mut args = CliConfig::parse();
    let mut config = SimpleCommitsConfig::default();

    // load custom path if exists or load global config
    // check if the config path was provided.
    let config_path = if let Some(path) = args.config {
        path
    } else {
        let path = BaseDirs::new().unwrap().config_dir().join("sc");
        // create directory if not exists
        let _ = std::fs::create_dir_all(path.clone());

        path.join("config.toml")
    };

    // Load Global
    if let Ok(cfg_content) = std::fs::read_to_string(config_path) {
        let mut g_config: SimpleCommitsConfig = toml::from_str(&cfg_content).unwrap();
        config.merge(&mut g_config);
    }

    // Load Local
    // TODO: make recursive search root path
    // The idea is find `.git` folder and use that as root path
    if let Ok(cfg_content) = std::fs::read_to_string(
        current_dir()
            .expect("Cannot get current directory")
            .join("zippy.toml"),
    ) {
        let mut l_config: SimpleCommitsConfig = toml::from_str(&cfg_content).unwrap();
        config.merge(&mut l_config);
    }

    // Merge arguments to loaded config
    config.merge(&mut args.sc_config);

    config
}
