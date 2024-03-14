mod config;
mod helpers;
mod steps;
mod structs;

pub fn init() {
    let config = config::generate_config();
    inquire::set_global_render_config(config);

    steps::init();
}

#[derive(Default, Debug)]
pub struct State {
    _type: String,
}

pub trait Step {
    fn run(&self, state: &mut State);
}

struct Steps<T: Step> {
    steps: Vec<T>,
}
