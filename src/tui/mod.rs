mod config;
mod helpers;
mod structs;

pub fn init() {
    let config = config::generate_config();
    inquire::set_global_render_config(config);
}
