use crate::{
    errors::AppError,
    reimpl::Pipeline,
    tui::{git, style},
};
use log::info;
use nobubbles::inline::{multiselect, task};

/// `XY path`, porcelain v1. A rename reads `old -> new`, and it is the new name that stages.
fn path_of(line: &str) -> String {
    let path = line.get(3..).unwrap_or(line).trim();
    path.rsplit(" -> ")
        .next()
        .unwrap_or(path)
        .trim_matches('"')
        .to_owned()
}

/// A short, human note for a porcelain v1 status code, shown as the picker's aside.
fn describe(line: &str) -> &'static str {
    match line.get(..2).unwrap_or("") {
        "??" => "untracked",
        "A " | "AM" => "added",
        " D" | "D " => "deleted",
        "R " | "RM" => "renamed",
        "C " => "copied",
        "AA" | "UU" | "DD" => "conflict",
        _ => "modified",
    }
}

/// Step 0: File Selection
///
/// Lists every changed path, tracked or not, and stages only the ones picked.
///
/// Unlike a `--dry-run`, this really runs `git add`, so ticking an untracked file stages it
/// for real — there's no separate "add this new file" step to remember.
pub fn select_files(_pipeline: &mut Pipeline) -> Result<(), AppError> {
    let changes = git::quietly(&["status", "--porcelain"]);

    if changes.is_empty() {
        nobubbles::inline::log::warn("nothing to commit, the working tree is clean");
        return Err(AppError::Step("nothing to commit".to_owned()));
    }

    let mut picker = multiselect(style::subtitle("What goes in the commit?"))
        .items(changes.iter().map(|line| path_of(line)))
        .max_rows(10);

    for (idx, line) in changes.iter().enumerate() {
        picker = picker.note(idx, describe(line));
    }

    let chosen = picker.ask()?;

    if chosen.is_empty() {
        nobubbles::inline::log::warn("nothing picked, nothing staged");
        return Err(AppError::Step("nothing picked".to_owned()));
    }

    let paths: Vec<String> = chosen.iter().map(|&at| path_of(&changes[at])).collect();

    let mut add = vec!["git".to_owned(), "add".to_owned(), "--".to_owned()];
    add.extend(paths.iter().cloned());

    let count = paths.len();
    let listing = paths.join(", ");

    let status = task(style::subtitle("Staging"), move |report| {
        report.say(format!("git add -- {listing}"));
        git::run(&add, report)
    })??;

    if !status.success() {
        nobubbles::inline::log::error(format!("git add exited with {status}"));
        return Err(AppError::Step("staging failed".to_owned()));
    }

    nobubbles::inline::log::success(format!(
        "staged {count} file{}",
        if count == 1 { "" } else { "s" }
    ));

    info!(target: "tui::steps::stage", "staged {count} file(s): {paths:?}");
    Ok(())
}
