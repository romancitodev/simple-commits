mod config;
mod gen;
mod git;
mod tui;

pub fn main() {
    config::start_logging();
    tui::init();
}
