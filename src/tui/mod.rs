use std::io::Stderr;

use crate::config::{get_config, SimpleCommitsConfig};

pub mod config;
pub mod config_prompt;
pub mod helpers;
pub mod steps;
pub mod structs;
pub mod widgets;

use config as tui;
use promptuity::{Error, Promptuity};

/// initialize the configuration and setup the steps
pub fn init() {
    let (mut term, mut theme) = tui::prepare();
    let mut prompt = tui::generate_prompt(&mut term, &mut theme);
    let (mut config, command) = get_config();
    match command {
        Some(_) => {
            _ = config_prompt::init(&mut prompt, &mut config);
        }
        None => {
            _ = steps::init(&mut prompt, &mut config);
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct State {
    pub commit: CommitBuilder,
    pub action: Action,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub enum Action {
    #[default]
    None,
    DryRun(String),
    Commit(String, Vec<String>),
}

impl Action {
    /// Returns the action to be executed of this [`Action`].
    pub fn execute_action(&self) {
        match self {
            Self::DryRun(msg) => println!("{msg}"),
            Self::Commit(cmd, args) => {
                std::process::Command::new(cmd)
                    .args(&args[..])
                    .spawn()
                    .unwrap();
            }
            _ => {}
        }
    }
}

pub type StepResult = Result<(), Error>;

/// A trait to setup steps along the TUI app.
pub trait Step {
    fn run(
        &self,
        prompt: &mut Promptuity<Stderr>,
        state: &mut State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult;
}

#[macro_export]
macro_rules! gen_steps {
    ($($module:ident),*) => {
        {
            let steps: Vec<Box<dyn super::Step>> = vec![
                $(
                    Box::new(self::$module::_Step),
                )*
            ];
            steps
        }
    };
}

#[derive(Debug, Default, Clone)]
pub struct CommitBuilder {
    r#type: Option<String>,
    scope: Option<String>,
    emoji: Option<String>,
    title: Option<String>,
    description: Option<String>,
    footer: Option<String>,
    is_breaking_change: Option<bool>,
    breaking_change_message: Option<String>, // This will filled if is_breaking_change is true
}

pub struct Commit(String);

pub enum BuildError {
    TypeRequired,
    TitleRequired,
}

impl CommitBuilder {
    pub fn set_type(&mut self, r#type: Option<String>) {
        self.r#type = r#type;
    }

    pub fn set_scope(&mut self, scope: Option<String>) {
        self.scope = scope;
    }

    pub fn set_emoji(&mut self, emoji: Option<String>) {
        self.emoji = emoji;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_footer(&mut self, footer: Option<String>) {
        self.footer = footer;
    }

    pub fn set_is_breaking_change(&mut self, is_breaking_change: Option<bool>) {
        self.is_breaking_change = is_breaking_change;
    }

    pub fn set_breaking_change_message(&mut self, breaking_change_message: Option<String>) {
        self.breaking_change_message = breaking_change_message;
    }

    pub fn build(self) -> Result<Commit, BuildError> {
        let CommitBuilder {
            r#type,
            title,
            scope,
            emoji,
            description,
            footer,
            is_breaking_change,
            breaking_change_message,
        } = self;

        if r#type.is_none() {
            log::error!("Type of commit required");
            return Err(BuildError::TypeRequired);
        };

        if title.is_none() {
            log::error!("Title of the commit required");
            return Err(BuildError::TitleRequired);
        }

        let r#type = r#type.unwrap();
        let title = title.unwrap();
        let scope = scope.map_or(String::new(), |s| format!("({s})"));
        let emoji = emoji.map_or(" ".to_owned(), |e| format!(" {e} "));
        let description = description.unwrap_or("\n".to_owned());
        let footer = footer.unwrap_or_default();
        let exclamation = is_breaking_change.map_or(String::new(), |_| "!".to_owned());
        let breaking_change_message = breaking_change_message.map_or(String::new(), |m| {
            format!("BREAKING CHANGE: {m}").trim().to_string()
        });

        let commit = format!(
            "{type}{scope}{exclamation}:{emoji}{title}

            {description}

            {breaking_change_message}
            {footer}
            "
        )
        .trim()
        .to_string();

        Ok(Commit(commit))
    }
}

impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        match value {
            BuildError::TypeRequired => Error::Prompt("Type required".to_owned()),
            BuildError::TitleRequired => Error::Prompt("Title required".to_owned()),
        }
    }
}
