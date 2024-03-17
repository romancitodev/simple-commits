use colored::Colorize;

pub trait Builder<T> {
    fn build(self) -> T;
}

#[derive(Clone)]
pub struct Commit {
    pub _type: String,
    pub emoji: Option<String>,
    pub scope: Option<String>,
    pub msg: String,
}

pub struct ColoredCommit(Commit);

#[derive(Default)]
pub struct CommitBuilder {
    _type: Option<String>,
    emoji: Option<String>,
    scope: Option<String>,
    msg: Option<String>,
}

impl CommitBuilder {
    pub fn set_type(self, _type: String) -> CommitBuilder {
        Self {
            _type: Some(_type),
            ..self
        }
    }
    pub fn set_emoji(self, emoji: String) -> CommitBuilder {
        Self {
            emoji: Some(emoji),
            ..self
        }
    }
    pub fn set_scope(self, scope: String) -> CommitBuilder {
        Self {
            scope: Some(scope),
            ..self
        }
    }
    pub fn set_msg(self, msg: String) -> CommitBuilder {
        Self {
            msg: Some(msg),
            ..self
        }
    }
}

impl Builder<Commit> for CommitBuilder {
    fn build(self) -> Commit {
        let Self {
            _type: Some(_type),
            emoji,
            scope,
            msg: Some(msg),
        } = self
        else {
            panic!("Cannot build because the type or msg of the commmit wasn't assigned")
        };
        Commit {
            _type,
            emoji,
            scope,
            msg,
        }
    }
}

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            _type,
            emoji,
            scope,
            msg,
        } = self;
        let emoji = emoji.clone().map_or(String::new(), |e| format!(" {e} "));
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
            .map_or(String::new(), |e| format!(" {e} "))
            .bright_blue();
        let scope = scope
            .clone()
            .map_or(String::new(), |s| format!(" {} ", s.bright_green()));
        write!(f, "{}{scope}:{emoji}{}", _type.bright_cyan(), msg.white())
    }
}
