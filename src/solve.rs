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

/// This uses iterative-bounded DFS (i.e. the stupidest possible IDA* variant) to find an optimal
/// solution to positionally solving a pocket cube
pub fn optimal_solve_position(arr: CubeletArrangement) -> Vec<Move> {
    fn find_solution(arr: CubeletArrangement, running: &mut Vec<Move>, fuel: usize) -> bool {
        if arr.is_solved() {
            return true;
        }

        if fuel == 0 {
            return false;
        }

        for dir in [Dir::R, Dir::U, Dir::F] {
            if running.last().map(|r| r.dir) == Some(dir) {
                continue;
            }

            // amt 1, 2, 3 means "move, move2, move_rev"
            for amt in [Amt::One, Amt::Two, Amt::Rev] {
                let m = Move { dir, amt };
                let moved = arr.clone().apply(m);
                running.push(m);
                let found = find_solution(moved, running, fuel - 1);
                if found {
                    return true;
                }
                running.pop();
            }
        }

        false
    }

    // it is known that every pocket cube can be solved in 11 moves so if we can't fix this
    // there is really something wrong with the cube
    const MAX_FUEL: usize = 13;

    for fuel in 0..MAX_FUEL {
        let mut running = Vec::with_capacity(fuel);

        let found = find_solution(arr.clone(), &mut running, fuel);

        if found {
            return running;
        }
    }

    unreachable!("Everything should be solvable in 11 moves, right")
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

#[cfg(test)]
mod pos_solve_tests {
    use super::*;
    use crate::cube::{Cube, Facelet};

    fn do_pos_solve_test(cube: Cube) -> Vec<Move> {
        let arr = cube.clone().make_pos_arr_from_dlb();

        let soln = optimal_solve_position(arr.clone());

        assert!(soln.len() < 12);

        if arr.is_solved() {
            assert_eq!(soln, vec![]);
            return soln;
        }

        let mut running = arr.clone();
        let mut running_cube = cube.clone();

        for m in soln.iter().copied() {
            assert!(!running.is_solved());

            running = running.apply(m);
            running_cube = running_cube.apply(m);
        }

        assert!(running.is_solved());

        assert!(running_cube.make_pos_arr_from_dlb().is_solved());

        return soln;
    }

    #[test]
    fn noop_solve() {
        let c = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        let soln = do_pos_solve_test(c);

        assert_eq!(soln, vec![]);
    }

    #[test]
    fn simple_solve() {
        let c = Cube::make_solved(Facelet::Green, Facelet::Yellow).right_two();

        let soln = do_pos_solve_test(c);

        assert_eq!(
            soln,
            vec![Move {
                dir: Dir::R,
                amt: Amt::Two
            }]
        );
    }

    #[test]
    fn two_move() {
        let c = Cube::make_solved(Facelet::Green, Facelet::Yellow)
            .right_two()
            .front_rev();

        let soln = do_pos_solve_test(c);

        // actually more than one optimal solution here, not gonna assert on the exact match
        assert_eq!(soln.len(), 2);
    }

    #[test]
    fn complex_move() {
        // random bunch of moves, no significance
        let c = Cube::make_solved(Facelet::Red, Facelet::White)
            .right_rev()
            .up_two()
            .front_rev()
            .right_two()
            .front_rev()
            .right_rev()
            .up_two()
            .right()
            .front_rev()
            .up()
            .front()
            .up();

        do_pos_solve_test(c);
    }
}
