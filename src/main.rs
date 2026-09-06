mod config;
pub mod errors;
mod gitmoji;
mod reimpl;
mod tui;

pub fn main() {
    config::start_logging();
    tui::init();
}
