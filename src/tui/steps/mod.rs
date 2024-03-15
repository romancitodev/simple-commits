use super::{State, Step};
use crate::gen_steps;
use colored::Colorize;

mod commit;
mod message;

pub fn init() {
    let mut state = State::default();
    let steps = gen_steps![commit, message];

    for step in steps {
        let res = step.run(&mut state);
        println!("{state:?}");
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            println!("{}", msg.bright_red());
            std::process::exit(1);
        }
    }
}
