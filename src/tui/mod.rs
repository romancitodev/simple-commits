mod config;
mod helpers;
mod steps;
mod structs;

/// initialize the configuration and setup the steps
pub fn init() {
    let config = config::generate_config();
    inquire::set_global_render_config(config);

    steps::init();
}

#[derive(Default, Debug)]
pub struct State {
    _type: String,
    emoji: Option<String>,
    msg: String,
}

#[derive(Debug)]
pub enum StepError {
    InvalidMsg,
    NoMsg,
    NoCommit,
}

pub type StepResult = Result<(), StepError>;

/// A trait to setup steps along the TUI app.
pub trait Step {
    fn run(&self, state: &mut State) -> StepResult;
}

#[macro_export]
macro_rules! gen_steps {
    ($($module:ident),*) => {
        {
            let steps: Vec<Box<dyn Step>> = vec![
                $(
                    Box::new(self::$module::_Step),
                )*
            ];
            steps
        }
    };
}
