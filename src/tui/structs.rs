use inquire::Autocomplete;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Commit<'c> {
    pub emoji: char,
    pub label: &'c str,
    pub hint: &'c str,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Scope {
    scopes: Vec<InnerScope>,
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

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl std::fmt::Display for InnerScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let description = self.description.clone().unwrap_or_default();
        writeln!(f, "{name} ({description})")
    }
}

#[derive(Clone)]
pub struct ScopesAutoComplete {
    scopes: Vec<InnerScope>,
}

impl Scope {
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

impl From<Scope> for ScopesAutoComplete {
    fn from(Scope { scopes }: Scope) -> Self {
        Self { scopes }
    }
}

impl ScopesAutoComplete {
    pub fn get_scope(&self, input: &str) -> Option<&InnerScope> {
        self.scopes.iter().find(|s| {
            s.name
                .to_lowercase()
                .trim()
                .contains(input.to_lowercase().trim())
        })
    }

    pub fn filter_scopes(&mut self, input: &str) -> Vec<String> {
        self.scopes
            .iter()
            .filter(|s| {
                s.name
                    .to_lowercase()
                    .trim()
                    .contains(input.to_lowercase().trim())
            })
            .map(|s| s.name.to_string())
            .collect()
    }
}

impl Autocomplete for ScopesAutoComplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        Ok(self.filter_scopes(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        Ok(self
            .get_scope(input)
            .map(|s| s.name.to_string())
            .or(highlighted_suggestion))
    }
}

impl<'c> Commit<'c> {
    pub const fn new(emoji: char, label: &'c str, hint: &'c str) -> Commit<'c> {
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

impl<'c> std::fmt::Display for Commit<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { emoji, label, hint } = self;
        writeln!(f, "{emoji} {label} ({hint})")
    }
}
