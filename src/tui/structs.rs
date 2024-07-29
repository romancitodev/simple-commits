use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default)]
pub struct Commit<'scope> {
    pub emoji: char,
    pub label: &'scope str,
    pub hint: &'scope str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Scope {
    scopes: Vec<InnerScope>,
}

impl Scope {
    pub fn scopes(&self) -> &Vec<InnerScope> {
        &self.scopes
    }

    pub fn exists(&self, scope: &str) -> bool {
        self.scopes
            .iter()
            .any(|s| s.name.to_lowercase().trim() == scope.to_lowercase().trim())
    }

    pub fn add_scope(&mut self, scope: String) {
        self.scopes.push(InnerScope {
            name: scope,
            description: None,
        });
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct InnerScope {
    name: String,
    description: Option<String>,
}

impl InnerScope {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

impl std::fmt::Display for InnerScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let description = self.description.clone().unwrap_or_default();
        writeln!(f, "{name} ({description})")
    }
}

impl<'scope> Commit<'scope> {
    pub const fn new(emoji: char, label: &'scope str, hint: &'scope str) -> Commit<'scope> {
        Self { emoji, label, hint }
    }
}

pub const COMMIT_TYPES: [Commit; 9] = [
    Commit::new('\u{2728}', "feat", "A new feature"),
    Commit::new('\u{1F41B}', "fix", "A bug fix"),
    Commit::new('\u{1F4DA}', "docs", "Documentation changes only"),
    Commit::new(
        '\u{1F4D5}',
        "refactor",
        "A code that neither fixes a bug or add a feature",
    ),
    Commit::new(
        '\u{1F9EA}',
        "test",
        "Adding missing tests or correcting existing tests",
    ),
    Commit::new(
        '\u{1F527}',
        "build",
        "Changes that affect the build system or external dependencies",
    ),
    Commit::new(
        '\u{1F916}',
        "ci",
        "Changes to our CI configuration files and scripts",
    ),
    Commit::new(
        '\u{1F9F9}',
        "chore",
        "Other changes that do not modify src or test files",
    ),
    Commit::new(
        '\u{1F680}',
        "perf",
        "A code change that improves performance",
    ),
];

impl<'scope> std::fmt::Display for Commit<'scope> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { emoji, label, .. } = self;
        write!(f, "{emoji} {label}")
    }
}
