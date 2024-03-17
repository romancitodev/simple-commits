use super::{State, Step};
use crate::{config::SimpleCommitsConfig, gen_steps};
use colored::Colorize;

mod commit;
mod emoji;
mod message;
mod scopes;

pub fn init(config: &mut SimpleCommitsConfig) -> State {
    let mut state = State::default();
    let steps = gen_steps![commit, scopes, emoji, message];

    for step in steps {
        let res = step.run(&mut state, config);
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            println!("{}", msg.bright_red());
            std::process::exit(1);
        }
    }

    state
}
