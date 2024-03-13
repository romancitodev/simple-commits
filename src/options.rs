#![allow(dead_code)]

use colored::*;
use inquire::list_option::ListOption;
use sonic_rs::Deserialize;

const EMOJI_URL: &str = "https://raw.githubusercontent.com/carloscuesta/gitmoji/v3.14.0/packages/gitmojis/src/gitmojis.json";

#[derive(Deserialize)]
struct UrlData {
    gitmojis: Vec<Emoji>,
}

#[derive(Deserialize, Clone)]
pub struct Emoji {
    emoji: String,
    description: String,
    name: String,
}

impl Emoji {
    pub fn empty() -> Emoji {
        Self::new("", "none", "")
    }

    pub fn new(emoji: &str, description: &str, name: &str) -> Emoji {
        Emoji {
            emoji: emoji.to_owned(),
            description: description.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn emoji(&self) -> String {
        self.emoji.to_string()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("({})", self.name).bright_blue();
        write!(f, "{} | {} {}", self.emoji, self.description, name)
    }
}

#[derive(Clone)]
pub struct CommitType<'e> {
    emoji: char,
    label: &'e str,
    hint: &'e str,
}

impl<'c> CommitType<'c> {
    pub const fn new(emoji: char, label: &'c str, hint: &'c str) -> CommitType<'c> {
        CommitType { emoji, label, hint }
    }

    pub fn label(&self) -> &str {
        self.label
    }

    pub fn hint(&self) -> &str {
        self.hint
    }

    pub fn emoji(&self) -> char {
        self.emoji
    }
}

impl<'e> std::fmt::Display for CommitType<'e> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hint = format!("({})", self.hint).bright_black();
        write!(f, "{} | {:<10} {}", self.emoji, self.label, hint)
    }
}

pub const COMMIT_TYPES: [CommitType; 9] = [
    CommitType::new('\u{2728}', "feat", "A new feature"),
    CommitType::new('\u{1F41B}', "fix", "A bug fix"),
    CommitType::new('\u{1F4DA}', "docs", "Documentation changes only"),
    CommitType::new(
        '\u{1F4D5}',
        "refactor",
        "A code that neither fixes a bug or add a feature",
    ),
    CommitType::new(
        '\u{1F9EA}',
        "test",
        "Adding missing tests or correcting existing tests",
    ),
    CommitType::new(
        '\u{1F527}',
        "build",
        "Changes that affect the build system or external dependencies",
    ),
    CommitType::new(
        '\u{1F916}',
        "ci",
        "Changes to our CI configuration files and scripts",
    ),
    CommitType::new(
        '\u{1F9F9}',
        "chore",
        "Other changes that do not modify src or test files",
    ),
    CommitType::new(
        '\u{1F680}',
        "perf",
        "A code change that improves performance",
    ),
];

pub fn format_commits(list: ListOption<&CommitType<'_>>) -> String {
    let label = list.value.label();
    let correct = '\u{2705}';
    format!("{label} -- {correct}")
}

pub fn format_emojis(list: ListOption<&Emoji>) -> String {
    let label = list.value.emoji();
    let correct = '\u{2705}';
    format!("{label} -- {correct}")
}

pub async fn fetch_emojis() -> Vec<Emoji> {
    let url_data = reqwest::get(EMOJI_URL)
        .await
        .unwrap()
        .json::<UrlData>()
        .await;
    let res = url_data.unwrap();
    res.gitmojis
}
