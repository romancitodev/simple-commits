mod config;
pub mod errors;
mod gen;
mod reimpl;
mod tui;

pub fn main() {
    config::start_logging();
    tui::init();
}
