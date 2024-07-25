// use std::error::Error;

// use inquire::list_option::ListOption;
// use inquire::validator::Validation;

// type ValidationError = Box<dyn Error + Send + Sync>;

// use crate::gen::Emoji;

// use crate::tui::structs::Commit;

// pub fn format_emojis(list: ListOption<&Emoji>) -> String {
//     let Emoji { emoji, .. } = list.value;
//     let correct = '\u{2705}';
//     format!("{emoji} | {correct}")
// }

// pub fn format_commits(list: ListOption<&Commit<'_>>) -> String {
//     let Commit { label, .. } = list.value;
//     let correct = '\u{2705}';
//     format!("{label} | {correct}")
// }

pub fn valid_length(text: &String, min: usize, msg: &str) -> Result<(), String> {
    if text.len() > min {
        Ok(())
    } else {
        Err(msg.to_owned())
    }
}
