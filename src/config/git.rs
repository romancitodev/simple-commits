use super::helpers::swap_option;
use clap::Parser;
use merge2::Merge;
use serde::{Deserialize, Serialize};

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
