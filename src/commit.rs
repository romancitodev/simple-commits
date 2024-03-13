use colored::*;

pub struct Commit {
    c_type: String,
    emoji: Option<String>,
    scope: Option<String>,
    message: String,
}

impl Commit {
    pub fn new(
        c_type: String,
        emoji: Option<String>,
        scope: Option<String>,
        message: String,
    ) -> Self {
        Self {
            c_type,
            emoji,
            scope,
            message,
        }
    }

    pub fn raw_commit(&self) -> String {
        let emoji = self
            .emoji
            .clone()
            .map_or(String::new(), |e| format!(" {e} "));
        let scope = &self
            .scope
            .clone()
            .map_or(String::new(), |s| format!("({s})"));

        // <type>(<scope>): <emoji> <title>
        format!("{}{}:{}{}", self.c_type, scope, emoji, self.message)
    }
}

impl Commit {
    fn to_string(&self) -> String {
        let emoji = self
            .emoji
            .clone()
            .map_or(String::new(), |c| format!(" {c} "));

        let scope = self
            .scope
            .clone()
            .map_or(String::new(), |c| format!("({c})"));

        format!("{}{}:{}{}", self.c_type, scope, emoji, self.message)
    }
}

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = self
            .emoji
            .clone()
            .map_or(" ".to_owned(), |e| format!(" {e} "));
        let scope = &self
            .scope
            .clone()
            .map_or(String::new(), |s| format!("({})", s.bright_green()));

        // <type>(<scope>): <emoji> <title>
        write!(
            f,
            "{}{}:{}{}",
            self.c_type.bright_blue(),
            scope,
            emoji,
            self.message.bright_yellow()
        )
    }
}
