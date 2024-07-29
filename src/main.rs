mod config;
mod gen;
mod tui;

pub fn main() {
    config::start_logging();
    tui::init();
}
