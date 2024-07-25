use std::io::Stderr;

use crate::config::{get_config, SimpleCommitsConfig};

pub mod config;
pub mod config_prompt;
pub mod helpers;
pub mod steps;
pub mod structs;
pub mod widgets;

use config as tui;
use promptuity::{Error, Promptuity};
use steps::exec::Action;

/// initialize the configuration and setup the steps
pub fn init() {
    let (mut term, mut theme) = tui::prepare();
    let mut prompt = tui::generate_prompt(&mut term, &mut theme);
    let (mut config, command) = get_config();
    match command {
        Some(_) => {
            _ = config_prompt::init(&mut prompt, &mut config);
        }
        None => {
            _ = steps::init(&mut prompt, &mut config);
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct State {
    pub _type: String,
    pub scope: Option<String>,
    pub emoji: Option<String>,
    pub msg: String,
    pub exec_type: Option<Action>,
}

pub type StepResult = Result<(), Error>;

/// A trait to setup steps along the TUI app.
pub trait Step {
    fn run(
        &self,
        prompt: &mut Promptuity<Stderr>,
        state: &mut State,
        config: &mut SimpleCommitsConfig,
    ) -> StepResult;
}

#[macro_export]
macro_rules! gen_steps {
    ($($module:ident),*) => {
        {
            let steps: Vec<Box<dyn super::Step>> = vec![
                $(
                    Box::new(self::$module::_Step),
                )*
            ];
            steps
        }
    };
}
