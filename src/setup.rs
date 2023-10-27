use crate::cube::Cube;

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
pub enum Amt {
    One,
    Two,
    Rev,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FullMove(FullDir, Amt);

pub fn parse_line(input: &str) -> Result<Vec<FullMove>, &str> {
    let mut out = Vec::new();

    for tok in input.split_ascii_whitespace() {
        let next: FullMove = FullMove::try_from(tok)?;
        out.push(next);
    }

    Ok(out)
}

impl<'a> TryFrom<&'a str> for FullMove {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use Amt::*;
        use FullDir::*;

        match value {
            "R" => Ok(FullMove(R, One)),
            "R2" => Ok(FullMove(R, Two)),
            "R'" => Ok(FullMove(R, Rev)),

            "L" => Ok(FullMove(L, One)),
            "L2" => Ok(FullMove(L, Two)),
            "L'" => Ok(FullMove(L, Rev)),

            "F" => Ok(FullMove(F, One)),
            "F2" => Ok(FullMove(F, Two)),
            "F'" => Ok(FullMove(F, Rev)),

            "B" => Ok(FullMove(B, One)),
            "B2" => Ok(FullMove(B, Two)),
            "B'" => Ok(FullMove(B, Rev)),

            "U" => Ok(FullMove(U, One)),
            "U2" => Ok(FullMove(U, Two)),
            "U'" => Ok(FullMove(U, Rev)),

            "D" => Ok(FullMove(D, One)),
            "D2" => Ok(FullMove(D, Two)),
            "D'" => Ok(FullMove(D, Rev)),

            other => Err(other),
        }
    }
}

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
