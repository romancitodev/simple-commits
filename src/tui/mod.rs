use crate::config::get_config;

pub mod config_prompt;
pub mod helpers;
pub mod steps;
pub mod structs;
pub mod widgets;

/// initialize the configuration and setup the steps
pub fn init() {
    let (config, command) = get_config();
    match command {
        Some(_) => {
            _ = config_prompt::init(config);
        }
        None => {
            _ = steps::init(config);
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AppData {
    pub commit: CommitBuilder,
}

/// Builder for constructing a conventional commit message.
///
/// This builder follows the Conventional Commits specification and supports:
/// - Commit type (feat, fix, docs, etc.)
/// - Optional scope
/// - Optional emoji
/// - Title/subject line
/// - Optional body/description
/// - Optional footer notes
/// - Breaking change markers and messages
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

/// The final commit message ready to be executed.
pub struct Commit(pub String);

/// Errors that can occur when building a commit message.
#[derive(Debug)]
pub enum BuildError {
    /// The commit type (feat, fix, etc.) is required but was not provided.
    TypeRequired,
    /// The commit title/subject is required but was not provided.
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
        }

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
        let exclamation = if let Some(true) = is_breaking_change {
            "!".to_owned()
        } else {
            String::new()
        };
        let breaking_change_message = breaking_change_message.map_or(String::new(), |m| {
            format!("BREAKING CHANGE: {m}").trim().to_string()
        });

        let commit = format!(
            "{type}{scope}{exclamation}:{emoji}{title}\n\n{description}\n\n{breaking_change_message}\n\n{footer}"
        )
        .trim()
        .to_string();

        Ok(Commit(commit))
    }
}
