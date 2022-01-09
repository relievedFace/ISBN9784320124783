use std::collections::VecDeque;

use crate::tm::{Delta, Program, Tape, D, Q, S};

pub fn eval((state, delta): &Program, tape: &mut Tape) {
    exec(delta, *state, tape);
}

fn exec(delta: &Delta, q: Q, tape: &mut Tape) {
    if let Some((_, (q1, s, d))) = delta.iter().find(|x| x.0 == (q, tape.h)) {
        tape.h = *s;
        r#move(*d, tape);
        exec(delta, *q1, tape);
    }
}

fn r#move(d: D, tape: &mut Tape) {
    match d {
        D::R => move_r(tape),
        D::L => move_l(tape),
    };
}

fn move_l(tape: &mut Tape) {
    if tape.l_list.is_none() {
        tape.l_list = Some(VecDeque::new());
    }
    if let Some(l_list) = tape.l_list.as_mut() {
        l_list.push_front(tape.h)
    }
    tape.h = if let Some(s) = tape
        .r_list
        .as_mut()
        .map(|r_list| r_list.pop_front())
        .flatten()
    {
        s
    } else {
        S::B
    };
}

fn move_r(tape: &mut Tape) {
    if tape.r_list.is_none() {
        tape.r_list = Some(VecDeque::new());
    }
    if let Some(r_list) = tape.r_list.as_mut() {
        r_list.push_front(tape.h);
    }
    tape.h = if let Some(s) = tape
        .l_list
        .as_mut()
        .map(|l_list| l_list.pop_front())
        .flatten()
    {
        s
    } else {
        S::B
    };
}
