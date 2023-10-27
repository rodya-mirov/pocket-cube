use crate::moves::Amt;
use crate::moves::FullDir;
use crate::moves::FullMove;

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
