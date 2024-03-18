use clap::Parser;
use directories::BaseDirs;
use merge2::Merge;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs;
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
#[derive(Default, Serialize, Deserialize, Parser, Merge)]
pub struct SimpleCommitsConfig {
    #[merge(skip)]
    #[serde(skip)]
    #[clap(skip)]
    config: PathBuf,

    #[clap(skip)]
    #[serde(flatten)]
    #[merge(strategy = swap_option)]
    pub scopes: Option<Scopes>,

    #[clap(flatten)]
    #[merge(strategy = swap_option)]
    pub git: Option<GitConfig>,
}

/// Configs to run git commands
#[derive(Clone, Default, Serialize, Deserialize, Parser, Merge)]
pub struct GitConfig {
    /// Confirm before to run git commit
    #[clap(long, short)]
    pub skip_preview: bool,

    /// Command to run after generate commit message
    #[clap(long, short)]
    #[merge(strategy = swap_option)]
    pub commit_template: Option<Vec<String>>,
}

impl SimpleCommitsConfig {
    pub fn update(&self) -> std::io::Result<()> {
        let updated = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.config, updated)?;
        Ok(())
    }
}

pub fn get_config() -> SimpleCommitsConfig {
    let mut args = CliConfig::parse();
    let mut config = SimpleCommitsConfig::default();

    // load custom path if exists or load global config
    // check if the config path was provided.

    let config_path = args.config.unwrap_or_else(|| {
        let path = BaseDirs::new().unwrap().config_dir().join("sc");
        // create directory if not exists
        let _res = std::fs::create_dir_all(path.clone()).unwrap();
        path.join("config.toml")
    });

    // Load Global
    if let Ok(cfg_content) = std::fs::read_to_string(&config_path) {
        let mut g_config: SimpleCommitsConfig = toml::from_str(&cfg_content).unwrap();
        config.merge(&mut g_config);
        config.config = config_path;
    }

    let config_path = current_dir()
        .expect("Cannot get current directory")
        .join("sc.toml");

    // Load Local
    // TODO: make recursive search root path
    // The idea is find `.git` folder and use that as root path
    if let Ok(cfg_content) = std::fs::read_to_string(&config_path) {
        let mut l_config: SimpleCommitsConfig = toml::from_str(&cfg_content).unwrap();
        config.merge(&mut l_config);
        config.config = config_path;
    }

    // Merge arguments to loaded config
    config.merge(&mut args.sc_config);
    config
}
