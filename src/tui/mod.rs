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
    msg: String,
}

#[derive(Debug)]
pub enum StepError {
    InvalidMsg,
}

pub type StepResult = Result<(), StepError>;

pub trait Step {
    fn run(&self, state: &mut State) -> StepResult;
}

struct Steps<T: Step> {
    steps: Vec<T>,
}

#[macro_export]
macro_rules! gen_steps {
    ($($module:ident),*) => {
        {
            let mut steps: Vec<Box<dyn Step>> = Vec::new();
            $(
                steps.push(Box::new(self::$module::_Step));
            )*
            steps
        }
    };
}
