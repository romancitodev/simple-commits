use super::{State, Step};
use crate::{config::FileConfig, gen_steps};
use colored::Colorize;

mod commit;
mod emoji;
mod message;
mod scopes;

/// TODO: Find the way to obtain the config.
/// Pass it as parameter or initialize it inside.
pub fn init(config: &mut FileConfig) -> State {
    let mut state = State::default();
    // let mut config = CliConfig::new();
    let steps = gen_steps![commit, emoji, message];

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
