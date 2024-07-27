use colored::Colorize;

#[derive(Clone)]
pub struct Commit {
    pub _type: String,
    pub emoji: Option<String>,
    pub scope: Option<String>,
    pub msg: String,
}

pub struct ColoredCommit(Commit);

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            _type,
            emoji,
            scope,
            msg,
        } = self;
        let emoji = emoji.clone().map_or(" ".into(), |e| format!(" {e} "));
        let scope = scope.clone().map_or(String::new(), |s| format!("({s})"));
        write!(f, "{_type}{scope}:{emoji}{msg}")
    }
}

impl From<Commit> for ColoredCommit {
    fn from(value: Commit) -> Self {
        ColoredCommit(value)
    }
}

impl std::fmt::Display for ColoredCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Commit {
            _type,
            emoji,
            scope,
            msg,
        } = self.0.clone();
        let emoji = emoji
            .clone()
            .map_or(" ".into(), |e| format!(" {e} "))
            .bright_blue();
        let scope = scope
            .clone()
            .map_or(String::new(), |s| format!("({})", s.bright_green()));
        write!(f, "{}{scope}:{emoji}{}", _type.bright_cyan(), msg.white())
    }
}
