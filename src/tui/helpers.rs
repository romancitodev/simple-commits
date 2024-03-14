use inquire::list_option::ListOption;

use crate::tui::structs::Emoji;

use crate::tui::structs::Commit;

pub fn format_emojis(list: ListOption<&Emoji>) -> String {
    let Emoji { emoji, .. } = list.value;
    let correct = '\u{2705}';
    format!("{emoji} | {correct}")
}

pub fn format_commits(list: ListOption<&Commit<'_>>) -> String {
    let Commit { label, .. } = list.value;
    let correct = '\u{2705}';
    format!("{label} | {correct}")
}
