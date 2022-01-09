use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum D {
    R,
    L,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum S {
    B,
    I,
    O,
}

impl Display for S {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            S::B => write!(f, "B"),
            S::I => write!(f, "I"),
            S::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Q {
    M,
    H,
}

pub type Delta<'a> = &'a [((Q, S), (Q, S, D))];

pub type Program<'a> = (Q, Delta<'a>);

#[derive(Debug)]
pub struct Tape {
    pub(crate) r_list: Option<VecDeque<S>>,
    pub(crate) h: S,
    pub(crate) l_list: Option<VecDeque<S>>,
}

impl Tape {
    pub fn new(r_list: Option<VecDeque<S>>, h: S, l_list: Option<VecDeque<S>>) -> Tape {
        Tape { r_list, h, l_list }
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let disp_r_list = match &self.r_list {
            Some(r_list) => r_list
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            None => "".to_string(),
        };

        let disp_l_list = match &self.l_list {
            Some(l_list) => l_list
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            None => "".to_string(),
        };

        writeln!(f, "[{}], {}, [{}]", disp_r_list, self.h, disp_l_list)
    }
}

pub const P: Program<'static> = (
    Q::M,
    &[
        ((Q::M, S::I), (Q::M, S::O, D::L)),
        ((Q::M, S::O), (Q::H, S::I, D::L)),
        ((Q::M, S::B), (Q::H, S::I, D::L)),
    ],
);
