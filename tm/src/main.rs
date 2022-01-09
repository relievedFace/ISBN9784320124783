pub mod eval;
pub mod tm;

use std::collections::VecDeque;

use eval::eval;
use tm::{Tape, P, S};

fn main() {
    let mut tape: Tape = Tape::new(Some(VecDeque::from([S::I, S::I, S::I])), S::I, None);
    print!("{}", tape);
    eval(&P, &mut tape);
    print!("{}", tape);
}
