use super::{State, Step};
use crate::gen_steps;

mod commit;
mod message;

pub fn init() {
    let mut state = State::default();
    let steps = gen_steps![commit, message];

    for step in steps {
        let res = step.run(&mut state);
        println!("{state:?}");
        if let Err(err) = res {
            panic!("Error {err:?}");
        }
    }
}
