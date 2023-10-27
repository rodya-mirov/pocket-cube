use std::fmt::{Display, Formatter};

use crate::cube::{Cube, CubeletArrangement};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir {
    R,
    U,
    F,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Amt {
    One,
    Two,
    Rev,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move {
    pub dir: Dir,
    pub amt: Amt,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.dir {
            Dir::R => write!(f, "R")?,
            Dir::U => write!(f, "U")?,
            Dir::F => write!(f, "F")?,
        }

        match self.amt {
            Amt::One => {}
            Amt::Two => write!(f, "2")?,
            Amt::Rev => write!(f, "'")?,
        }

        Ok(())
    }
}

pub fn nice_write(moves: &[Move]) -> String {
    if moves.is_empty() {
        return String::new();
    }

    let mut running = String::new();

    let mut iter = moves.iter();

    running.push_str(&format!("{}", iter.next().unwrap()));

    for i in iter {
        running.push_str(&format!(" {}", i));
    }

    running
}

pub trait CanMove: Sized {
    fn apply(self, m: Move) -> Self;

    fn apply_many(self, moves: &[Move]) -> Self {
        let mut running = self;
        for m in moves {
            running = running.apply(*m);
        }
        running
    }
}

impl CanMove for CubeletArrangement {
    #[inline(always)]
    fn apply(self, m: Move) -> CubeletArrangement {
        match m.dir {
            Dir::R => match m.amt {
                Amt::One => self.r(),
                Amt::Two => self.r_two(),
                Amt::Rev => self.r_rev(),
            },
            Dir::U => match m.amt {
                Amt::One => self.u(),
                Amt::Two => self.u_two(),
                Amt::Rev => self.u_rev(),
            },
            Dir::F => match m.amt {
                Amt::One => self.f(),
                Amt::Two => self.f_two(),
                Amt::Rev => self.f_rev(),
            },
        }
    }
}

impl CanMove for Cube {
    #[inline(always)]
    fn apply(self, m: Move) -> Cube {
        match m.dir {
            Dir::R => match m.amt {
                Amt::One => self.right(),
                Amt::Two => self.right_two(),
                Amt::Rev => self.right_rev(),
            },
            Dir::U => match m.amt {
                Amt::One => self.up(),
                Amt::Two => self.up_two(),
                Amt::Rev => self.up_rev(),
            },
            Dir::F => match m.amt {
                Amt::One => self.front(),
                Amt::Two => self.front_two(),
                Amt::Rev => self.front_rev(),
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FullDir {
    R,
    L,
    F,
    B,
    U,
    D,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FullMove(pub FullDir, pub Amt);

pub trait CanFullMove: Sized {
    fn apply_full(self, m: FullMove) -> Self;

    fn apply_many_full(self, moves: &[FullMove]) -> Self {
        let mut running = self;
        for m in moves {
            running = running.apply_full(*m);
        }
        running
    }
}

impl CanFullMove for Cube {
    fn apply_full(self, m: FullMove) -> Self {
        match m.0 {
            FullDir::R => match m.1 {
                Amt::One => self.right(),
                Amt::Two => self.right_two(),
                Amt::Rev => self.right_rev(),
            },
            FullDir::U => match m.1 {
                Amt::One => self.up(),
                Amt::Two => self.up_two(),
                Amt::Rev => self.up_rev(),
            },
            FullDir::F => match m.1 {
                Amt::One => self.front(),
                Amt::Two => self.front_two(),
                Amt::Rev => self.front_rev(),
            },
            FullDir::L => match m.1 {
                Amt::One => self.left(),
                Amt::Two => self.left_two(),
                Amt::Rev => self.left_rev(),
            },
            FullDir::B => match m.1 {
                Amt::One => self.back(),
                Amt::Two => self.back_two(),
                Amt::Rev => self.back_rev(),
            },
            FullDir::D => match m.1 {
                Amt::One => self.down(),
                Amt::Two => self.down_two(),
                Amt::Rev => self.down_rev(),
            },
        }
    }
}
