use inquire::{autocompletion::Replacement, Autocomplete};

pub struct Emoji {
    pub emoji: String,
    pub description: String,
    pub name: String,
}

#[derive(Clone)]
pub struct Commit<'c> {
    pub emoji: char,
    pub label: &'c str,
    pub hint: &'c str,
}

pub struct Scopes {
    scopes: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct ScopesAutoComplete {
    scopes: Vec<String>,
}

impl Scopes {
    pub fn exists(&self, scope: &str) -> bool {
        let Some(scopes) = &self.scopes else {
            return false;
        };
        scopes
            .iter()
            .any(|s| s.to_lowercase().trim() == scope.to_lowercase().trim())
    }

    pub fn add_scope(&mut self, scope: String) {
        let mut scopes = self.scopes.clone().unwrap_or_default();
        scopes.push(scope);
        self.scopes = Some(scopes);
    }
}

impl From<Scopes> for ScopesAutoComplete {
    fn from(Scopes { scopes }: Scopes) -> Self {
        Self {
            scopes: scopes.unwrap_or_default(),
        }
    }
}

impl ScopesAutoComplete {
    pub fn filter_scopes(&mut self, input: &str) -> Vec<String> {
        self.scopes
            .iter()
            .filter(|s| {
                s.to_lowercase()
                    .trim()
                    .contains(input.to_lowercase().trim())
            })
            .cloned()
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
        self.filter_scopes(input);
        Ok(highlighted_suggestion)
    }
}

impl<'c> Commit<'c> {
    pub const fn new(emoji: char, label: &'c str, hint: &'c str) -> Commit<'c> {
        Self { emoji, label, hint }
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
