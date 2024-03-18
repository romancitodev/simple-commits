use super::{State, Step};
use crate::git::commit::ColoredCommit;
use crate::{config::SimpleCommitsConfig, gen_steps};
use colored::Colorize;

mod commit;
mod emoji;
mod exec;
mod message;
mod scopes;

pub use exec::ExecType;

pub fn init(config: &mut SimpleCommitsConfig) {
    let mut state = State::default();
    let steps = gen_steps![commit, scopes, emoji, message, exec];

    for step in steps {
        let res = step.run(&mut state, config);
        if let Err(err) = res {
            let msg = format!("❌ Error: {:?}", err);
            println!("{}", msg.bright_red());
            std::process::exit(1);
        }
    }

    match state.exec_type {
        Some(ExecType::Message(msg)) => println!("{msg}"),
        Some(ExecType::Command(cmd, args)) => {
            std::process::Command::new(cmd)
                .args(&args[..])
                .spawn()
                .unwrap();
        }
        _ => {
            let commit: ColoredCommit = state.clone().into();
            println!("{commit}")
        }
    }
}
