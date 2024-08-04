use crate::tui::structs::Scope;

use super::git::GitConfig;
use super::helpers::swap_option;
use clap::{Parser, Subcommand};
use merge2::Merge;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Parser)]
/// Cli config settings.
pub struct CliConfig {
    #[clap(long, help = "set custom path to load config.")]
    pub(super) config: Option<PathBuf>,

    #[clap(flatten)]
    pub(super) sc_config: SimpleCommitsConfig,

    #[command(subcommand)]
    pub(super) mode: Option<Command>,
}

#[derive(Subcommand, Clone, Copy, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Init(InitOptions),
}

#[derive(Subcommand, Clone, Copy, Debug)]
pub enum InitOptions {
    Global,
    Local,
}

/// File settings for customizing the bin.
#[derive(Debug, Default, Serialize, Deserialize, Parser, Merge)]
pub struct SimpleCommitsConfig {
    #[merge(skip)]
    #[serde(skip)]
    #[clap(skip)]
    pub config: PathBuf,

    #[clap(short, long)]
    #[serde(skip)]
    pub message: Option<String>,

    #[clap(skip)]
    #[serde(flatten)]
    #[merge(strategy = swap_option)]
    pub scopes: Option<Scope>,

    #[clap(flatten)]
    #[merge(strategy = swap_option)]
    pub git: Option<GitConfig>,
}

impl SimpleCommitsConfig {
    pub fn update(&self) -> std::io::Result<()> {
        let updated = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.config, updated)?;
        Ok(())
    }
}
