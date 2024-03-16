use colored::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Emoji {
  pub emoji: &'static str,
  pub description: &'static str,
  pub name: &'static str,
}

impl Emoji {
  const fn new(emoji: &'static str, name: &'static str, description: &'static str) -> Self {
    Self {
        emoji,
        name,
        description,
    }
  }
}

impl std::fmt::Display for Emoji {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("({})", self.name).bright_blue();
    write!(f, "{} | {} {}", self.emoji, self.description, name)
  }
}

pub const EMOJIS: [Emoji; 73] = [
	Emoji::new("🎨", "art", "Improve structure / format of the code."),
	Emoji::new("⚡️", "zap", "Improve performance."),
	Emoji::new("🔥", "fire", "Remove code or files."),
	Emoji::new("🐛", "bug", "Fix a bug."),
	Emoji::new("🚑️", "ambulance", "Critical hotfix."),
	Emoji::new("✨", "sparkles", "Introduce new features."),
	Emoji::new("📝", "memo", "Add or update documentation."),
	Emoji::new("🚀", "rocket", "Deploy stuff."),
	Emoji::new("💄", "lipstick", "Add or update the UI and style files."),
	Emoji::new("🎉", "tada", "Begin a project."),
	Emoji::new("✅", "white-check-mark", "Add, update, or pass tests."),
	Emoji::new("🔒️", "lock", "Fix security or privacy issues."),
	Emoji::new("🔐", "closed-lock-with-key", "Add or update secrets."),
	Emoji::new("🔖", "bookmark", "Release / Version tags."),
	Emoji::new("🚨", "rotating-light", "Fix compiler / linter warnings."),
	Emoji::new("🚧", "construction", "Work in progress."),
	Emoji::new("💚", "green-heart", "Fix CI Build."),
	Emoji::new("⬇️", "arrow-down", "Downgrade dependencies."),
	Emoji::new("⬆️", "arrow-up", "Upgrade dependencies."),
	Emoji::new("📌", "pushpin", "Pin dependencies to specific versions."),
	Emoji::new("👷", "construction-worker", "Add or update CI build system."),
	Emoji::new("📈", "chart-with-upwards-trend", "Add or update analytics or track code."),
	Emoji::new("♻️", "recycle", "Refactor code."),
	Emoji::new("➕", "heavy-plus-sign", "Add a dependency."),
	Emoji::new("➖", "heavy-minus-sign", "Remove a dependency."),
	Emoji::new("🔧", "wrench", "Add or update configuration files."),
	Emoji::new("🔨", "hammer", "Add or update development scripts."),
	Emoji::new("🌐", "globe-with-meridians", "Internationalization and localization."),
	Emoji::new("✏️", "pencil2", "Fix typos."),
	Emoji::new("💩", "poop", "Write bad code that needs to be improved."),
	Emoji::new("⏪️", "rewind", "Revert changes."),
	Emoji::new("🔀", "twisted-rightwards-arrows", "Merge branches."),
	Emoji::new("📦️", "package", "Add or update compiled files or packages."),
	Emoji::new("👽️", "alien", "Update code due to external API changes."),
	Emoji::new("🚚", "truck", "Move or rename resources (e.g.: files, paths, routes)."),
	Emoji::new("📄", "page-facing-up", "Add or update license."),
	Emoji::new("💥", "boom", "Introduce breaking changes."),
	Emoji::new("🍱", "bento", "Add or update assets."),
	Emoji::new("♿️", "wheelchair", "Improve accessibility."),
	Emoji::new("💡", "bulb", "Add or update comments in source code."),
	Emoji::new("🍻", "beers", "Write code drunkenly."),
	Emoji::new("💬", "speech-balloon", "Add or update text and literals."),
	Emoji::new("🗃️", "card-file-box", "Perform database related changes."),
	Emoji::new("🔊", "loud-sound", "Add or update logs."),
	Emoji::new("🔇", "mute", "Remove logs."),
	Emoji::new("👥", "busts-in-silhouette", "Add or update contributor(s)."),
	Emoji::new("🚸", "children-crossing", "Improve user experience / usability."),
	Emoji::new("🏗️", "building-construction", "Make architectural changes."),
	Emoji::new("📱", "iphone", "Work on responsive design."),
	Emoji::new("🤡", "clown-face", "Mock things."),
	Emoji::new("🥚", "egg", "Add or update an easter egg."),
	Emoji::new("🙈", "see-no-evil", "Add or update a .gitignore file."),
	Emoji::new("📸", "camera-flash", "Add or update snapshots."),
	Emoji::new("⚗️", "alembic", "Perform experiments."),
	Emoji::new("🔍️", "mag", "Improve SEO."),
	Emoji::new("🏷️", "label", "Add or update types."),
	Emoji::new("🌱", "seedling", "Add or update seed files."),
	Emoji::new("🚩", "triangular-flag-on-post", "Add, update, or remove feature flags."),
	Emoji::new("🥅", "goal-net", "Catch errors."),
	Emoji::new("💫", "dizzy", "Add or update animations and transitions."),
	Emoji::new("🗑️", "wastebasket", "Deprecate code that needs to be cleaned up."),
	Emoji::new("🛂", "passport-control", "Work on code related to authorization, roles and permissions."),
	Emoji::new("🩹", "adhesive-bandage", "Simple fix for a non-critical issue."),
	Emoji::new("🧐", "monocle-face", "Data exploration/inspection."),
	Emoji::new("⚰️", "coffin", "Remove dead code."),
	Emoji::new("🧪", "test-tube", "Add a failing test."),
	Emoji::new("👔", "necktie", "Add or update business logic."),
	Emoji::new("🩺", "stethoscope", "Add or update healthcheck."),
	Emoji::new("🧱", "bricks", "Infrastructure related changes."),
	Emoji::new("🧑‍💻", "technologist", "Improve developer experience."),
	Emoji::new("💸", "money-with-wings", "Add sponsorships or money related infrastructure."),
	Emoji::new("🧵", "thread", "Add or update code related to multithreading or concurrency."),
	Emoji::new("🦺", "safety-vest", "Add or update code related to validation."),
];