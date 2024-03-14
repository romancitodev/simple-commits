pub struct Emoji {
    pub emoji: String,
    pub description: String,
    pub name: String,
}

pub struct Commit<'c> {
    pub emoji: char,
    pub label: &'c str,
    pub hint: &'c str,
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
