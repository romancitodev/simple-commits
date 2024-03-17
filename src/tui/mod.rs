use crate::{
    config::{get_config, SimpleCommitsConfig},
    git::commit::{ColoredCommit, Commit},
};

pub mod config;
pub mod helpers;
pub mod steps;
pub mod structs;

use config as tui_config;
/// initialize the configuration and setup the steps
pub fn init() {
    let render_config = tui_config::generate_tui_config();
    inquire::set_global_render_config(render_config);
    let mut config = get_config();
    let State {
        _type,
        emoji,
        scope,
        msg,
    } = steps::init(&mut config);

    let commit = Commit {
        _type,
        emoji,
        scope,
        msg,
    };

    // git::commit(commit);

    let colored: ColoredCommit = commit.into();

    println!("{colored}");
}

#[derive(Default, Debug)]
pub struct State {
    _type: String,
    scope: Option<String>,
    emoji: Option<String>,
    msg: String,
}

#[derive(Debug)]
pub enum StepError {
    InvalidMsg,
    NoMsg,
    NoCommit,
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
