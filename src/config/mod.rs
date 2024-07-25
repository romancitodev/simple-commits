use clap::{Parser, Subcommand};
use handle_config as helpers;
use merge2::Merge;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::tui::structs::Scope;

mod handle_config;

#[inline]
pub fn swap_option<T>(left: &mut Option<T>, right: &mut Option<T>) {
    if left.is_none() || right.is_some() {
        core::mem::swap(left, right);
    }
}

#[derive(Subcommand, Clone, Copy, Debug)]
pub enum InitOptions {
    Global,
    Local,
}

#[derive(Subcommand, Clone, Copy, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Init(InitOptions),
}

#[derive(Parser)]
/// Cli config settings.
pub struct CliConfig {
    #[clap(long, help = "set custom path to load config.")]
    config: Option<PathBuf>,

    #[clap(flatten)]
    sc_config: SimpleCommitsConfig,

    #[command(subcommand)]
    mode: Option<Command>,
}

/// File settings for customizing the bin.
#[derive(Default, Serialize, Deserialize, Parser, Merge)]
pub struct SimpleCommitsConfig {
    #[merge(skip)]
    #[serde(skip)]
    #[clap(skip)]
    pub config: PathBuf,

    #[clap(skip)]
    #[serde(flatten)]
    #[merge(strategy = swap_option)]
    pub scopes: Option<Scope>,

    #[clap(flatten)]
    #[merge(strategy = swap_option)]
    pub git: Option<GitConfig>,
}

/// Configs to run git commands
#[derive(Clone, Default, Serialize, Deserialize, Parser, Merge)]
pub struct GitConfig {
    /// Confirm before to run git commit
    #[arg(short = 'p', long = "skip-preview")]
    pub skip_preview: bool,

    /// Confirm before to run git commit
    #[arg(short = 'e', long = "skip-emojis")]
    pub skip_emojis: bool,

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

pub fn get_config() -> (SimpleCommitsConfig, Option<Command>) {
    let mut args = CliConfig::parse();
    let mut config = SimpleCommitsConfig::default();

    match args.mode {
        Some(Command::Init(option)) => {
            let path = helpers::create_config(option);
            config.config = path;
        }
        _ => {
            let (global_path, local_path) = helpers::get_config(args.config, &mut config);

            if let Some(local_path) = local_path {
                config.config = local_path;
            } else {
                config.config = global_path;
            }
            config.merge(&mut args.sc_config);
        }
    }
    (config, args.mode)
}

pub fn start_logging() {
    env_logger::builder()
        // .filter_level(log::LevelFilter::Trace)
        .init();
}
