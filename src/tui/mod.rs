use crate::config::{get_config, SimpleCommitsConfig};

pub mod config;
pub mod helpers;
pub mod steps;
pub mod structs;

use config as tui_config;
use inquire::InquireError;

use self::steps::ExecType;

/// initialize the configuration and setup the steps
pub fn init() {
    let render_config = tui_config::generate_tui_config();
    inquire::set_global_render_config(render_config);
    let mut config = get_config();
    steps::init(&mut config);
}

#[derive(Clone, Default, Debug)]
pub struct State {
    pub _type: String,
    pub scope: Option<String>,
    pub emoji: Option<String>,
    pub msg: String,
    pub exec_type: Option<ExecType>,
}

#[derive(Debug)]
pub enum StepError {
    InvalidMsg,
    NoMsg,
    NoCommit(InquireError),
    InvalidEmoji,
}

pub type StepResult = Result<(), StepError>;

/// A trait to setup steps along the TUI app.
pub trait Step {
    fn run(&self, state: &mut State, config: &mut SimpleCommitsConfig) -> StepResult;
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
