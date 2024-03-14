use self::commit::CommitStep;

use super::{State, Step};

mod commit;

pub fn init() {
    let mut state = State::default();
    let steps = vec![CommitStep::default()];

    for step in steps {
        step.run(&mut state);
        println!("{state:?}");
    }
}
