use std::collections::HashMap;

use crate::cube::{Cube, CubeletOrientationArrangement, CubeletPositionArrangement, Facelet};
use crate::moves::{Amt, CanMove, Dir, Move, reversed};
use crate::orr_solve::optimal_solve_orientation;
use crate::pos_solve::optimal_solve_position;

/// Describes which type of heuristic we will use for IDA* search
// In theory you can pick which heuristic type you want, but in practice there's no reason to,
// so I just hardcoded it to simplify the CLI. You can always mess with it later.
#[allow(unused)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum HeuristicType {
    None,
    Pos,
    Orr,
    PosAndOrr,
}

fn cache_helper_or_die<Arrangement: CanMove + Clone + std::hash::Hash + Eq + PartialEq>(
    cache: &HashMap<Arrangement, usize>,
    arr: Arrangement,
) -> usize {
    if let Some(dist) = cache.get(&arr) {
        return *dist;
    }

    panic!("Cannot fetch the needed arrangement, because there is no cached value for it")
}

fn cache_helper<
    Arrangement: CanMove + Clone + std::hash::Hash + Eq + PartialEq,
    Solver: FnOnce(Arrangement) -> Vec<Move>,
>(
    cache: &mut HashMap<Arrangement, usize>,
    arr: Arrangement,
    solve: Solver,
) -> usize {
    if let Some(dist) = cache.get(&arr) {
        return *dist;
    }

    let solution = solve(arr.clone());
    let full_length = solution.len();

    let mut running = arr;
    let mut remaining_length = solution.len();

    cache.insert(running.clone(), remaining_length);

    // it is a fact that if the shortest path from A to B is P, and P passes through C,
    // then the remaining path from C to B is also the shortest path from C to B
    // so this saves us running all 8! = 40320 possible position configurations, which I guess
    // is something
    for m in solution {
        running = running.apply(m);
        remaining_length -= 1;

        cache.insert(running.clone(), remaining_length);
    }

    full_length
}

pub trait Heuristic {
    fn estimated_remaining_cost(
        &mut self,
        pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize;

    fn estimate_or_die(
        &self,
        pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize;
}

pub trait ShortCircuitCache {
    fn learn_path(&mut self, cube: Cube, solution: &[Move]);

    fn known_solution(&self, cube: &Cube) -> Option<&Vec<Move>>;

    fn depth(&self) -> usize;

    fn load_with_depth(&mut self, depth: usize, f: Facelet, u: Facelet)
    where
        Self: Sized,
    {
        fn walk<S: ShortCircuitCache>(
            cube: Cube,
            cache: &mut S,
            running: &mut Vec<Move>,
            max_depth: usize,
        ) {
            match cache.known_solution(&cube) {
                None => cache.learn_path(cube.clone(), running),
                Some(existing) => {
                    if running.len() >= existing.len() {
                        return;
                    } else {
                        cache.learn_path(cube.clone(), running)
                    }
                }
            }

            if running.len() >= max_depth {
                return;
            }

            for dir in [Dir::R, Dir::F, Dir::U] {
                if running.last().map(|m| m.dir) == Some(dir) {
                    continue;
                }

                for amt in [Amt::One, Amt::Two, Amt::Rev] {
                    let m = Move { dir, amt };
                    let next_cube = cube.clone().apply(m);
                    running.push(m);
                    walk(next_cube, cache, running, max_depth);
                    running.pop();
                }
            }
        }

        let cube = Cube::make_solved(f.clone(), u.clone());

        walk(cube.clone(), self, &mut Vec::with_capacity(depth), depth);
    }
}

#[derive(Default)]
pub struct SimpleShortCircuitCache {
    cache: HashMap<Cube, Vec<Move>>,
    depth: usize,
}

impl SimpleShortCircuitCache {
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl ShortCircuitCache for SimpleShortCircuitCache {
    fn learn_path(&mut self, cube: Cube, solution: &[Move]) {
        let solution = reversed(solution).collect();
        self.cache.insert(cube, solution);
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn known_solution(&self, cube: &Cube) -> Option<&Vec<Move>> {
        self.cache.get(&cube)
    }
}

#[derive(Default)]
pub struct NoHeuristic;

impl Heuristic for NoHeuristic {
    fn estimated_remaining_cost(
        &mut self,
        _pos: CubeletPositionArrangement,
        _orr: CubeletOrientationArrangement,
    ) -> usize {
        0
    }

    fn estimate_or_die(
        &self,
        _pos: CubeletPositionArrangement,
        _orr: CubeletOrientationArrangement,
    ) -> usize {
        0
    }
}

#[derive(Default)]
pub struct PosHeuristic {
    pos_dist_cache: HashMap<CubeletPositionArrangement, usize>,
}

impl Heuristic for PosHeuristic {
    fn estimated_remaining_cost(
        &mut self,
        pos: CubeletPositionArrangement,
        _orr: CubeletOrientationArrangement,
    ) -> usize {
        cache_helper(&mut self.pos_dist_cache, pos, optimal_solve_position)
    }

    fn estimate_or_die(
        &self,
        pos: CubeletPositionArrangement,
        _orr: CubeletOrientationArrangement,
    ) -> usize {
        cache_helper_or_die(&self.pos_dist_cache, pos)
    }
}

#[derive(Default)]
pub struct OrrHeuristic {
    orr_dist_cache: HashMap<CubeletOrientationArrangement, usize>,
}

impl Heuristic for OrrHeuristic {
    fn estimated_remaining_cost(
        &mut self,
        _pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize {
        cache_helper(&mut self.orr_dist_cache, orr, optimal_solve_orientation)
    }

    fn estimate_or_die(
        &self,
        _pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize {
        cache_helper_or_die(&self.orr_dist_cache, orr)
    }
}

#[derive(Default)]
pub struct FullHeuristic {
    pos_dist_cache: HashMap<CubeletPositionArrangement, usize>,
    orr_dist_cache: HashMap<CubeletOrientationArrangement, usize>,
}

impl Heuristic for FullHeuristic {
    fn estimated_remaining_cost(
        &mut self,
        pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize {
        let a = cache_helper(&mut self.orr_dist_cache, orr, optimal_solve_orientation);
        let b = cache_helper(&mut self.pos_dist_cache, pos, optimal_solve_position);

        a.max(b)
    }

    fn estimate_or_die(
        &self,
        pos: CubeletPositionArrangement,
        orr: CubeletOrientationArrangement,
    ) -> usize {
        let a = cache_helper_or_die(&self.orr_dist_cache, orr);
        let b = cache_helper_or_die(&self.pos_dist_cache, pos);

        a.max(b)
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum SolveResult {
    // found a solution
    Success,
    // failed to find a solution
    Failed,
}

pub fn optimal_solve_heuristic<H: Heuristic, S: ShortCircuitCache>(
    cube: Cube,
    heuristic: &mut H,
    short_circuit_cache: &S,
) -> Vec<Move> {
    if cube.solved() {
        return vec![];
    }

    fn solve<H: Heuristic, S: ShortCircuitCache>(
        cube: Cube,
        pos_arr: CubeletPositionArrangement,
        orr_arr: CubeletOrientationArrangement,
        heuristic: &mut H,
        running: &mut Vec<Move>,
        max_cost: usize,
        short_circuit_cache: &S,
    ) -> SolveResult {
        if cube.solved() {
            return SolveResult::Success;
        } else if running.len() == max_cost {
            return SolveResult::Failed;
        }

        if let Some(known) = short_circuit_cache.known_solution(&cube) {
            if known.len() + running.len() <= max_cost {
                for m in known {
                    running.push(*m);
                }
                return SolveResult::Success;
            } else {
                return SolveResult::Failed;
            }
        } else if short_circuit_cache.depth() + running.len() >= max_cost {
            return SolveResult::Failed;
        }

        let heuristic_cost_now = heuristic.estimated_remaining_cost(pos_arr.clone(), orr_arr.clone());
        let est_total_cost_now = running.len() + heuristic_cost_now;

        for dir in [Dir::F, Dir::R, Dir::U] {
            if running.last().map(|m| m.dir) == Some(dir) {
                continue;
            }

            for amt in [Amt::One, Amt::Two, Amt::Rev] {
                let m = Move { amt, dir };
                running.push(m);

                let next_cube = cube.clone().apply(m);

                if next_cube.solved() {
                    return SolveResult::Success;
                }

                let next_pos_arr = pos_arr.clone().apply(m);
                let next_orr_arr = orr_arr.clone().apply(m);

                let heuristic_cost = heuristic
                    .estimated_remaining_cost(next_pos_arr.clone(), next_orr_arr.clone());

                let est_cost = running.len() + heuristic_cost;

                assert!(est_cost >= est_total_cost_now, "Heuristic cost must not drop too quickly");

                if est_cost <= max_cost {
                    // if we have enough gas to get to the next node, try it out
                    let iterate_result = solve(
                        next_cube,
                        next_pos_arr,
                        next_orr_arr,
                        heuristic,
                        running,
                        max_cost,
                        short_circuit_cache,
                    );

                    match iterate_result {
                        // immediately return, so the "running" vec has all the stuff it needs
                        SolveResult::Success => return SolveResult::Success,
                        SolveResult::Failed =>{},
                    }
                }

                running.pop();
            }
        }

        SolveResult::Failed
    }

    fn solve_with_heuristic<H: Heuristic, S: ShortCircuitCache>(
        cube: Cube,
        heuristic: &mut H,
        max_fuel: usize,
        short_circuit_cache: &S,
    ) -> Vec<Move> {
        let pos_arr = cube.clone().make_pos_arr_from_dlb();
        let orr_arr = cube.clone().make_orr_arr_from_dlb();

        let mut starting_fuel = 0;

        while starting_fuel < max_fuel {
            let mut running = Vec::new();

            let sr = solve(
                cube.clone(),
                pos_arr.clone(),
                orr_arr.clone(),
                heuristic,
                &mut running,
                starting_fuel,
                short_circuit_cache,
            );

            if sr == SolveResult::Success {
                return running;
            }

            starting_fuel += 1;
        }

        unreachable!("Should have found a solution!")
    }

    const MAX_FUEL: usize = 13;

    solve_with_heuristic(cube, heuristic, MAX_FUEL, short_circuit_cache)
}

pub fn optimal_solve(cube: Cube, heuristic_type: HeuristicType) -> Vec<Move> {
    let mut short_circuit_cache = SimpleShortCircuitCache::default();
    let des = cube.clone().make_desired_from_dlb();

    // right now, the heuristics and the short-circuit cache interact badly, so we disable
    // one or the other
    let short_circuit_depth = match heuristic_type {
        HeuristicType::None => 5,
        _ => 5,
    };

    println!("Precomputing cache of depth {}", short_circuit_depth);
    short_circuit_cache.load_with_depth(short_circuit_depth, des.f, des.u);

    match heuristic_type {
        HeuristicType::None => {
            optimal_solve_heuristic(cube, &mut NoHeuristic::default(), &short_circuit_cache)
        }
        HeuristicType::Pos => {
            optimal_solve_heuristic(cube, &mut PosHeuristic::default(), &short_circuit_cache)
        }
        HeuristicType::Orr => {
            optimal_solve_heuristic(cube, &mut OrrHeuristic::default(), &short_circuit_cache)
        }
        HeuristicType::PosAndOrr => {
            optimal_solve_heuristic(cube, &mut FullHeuristic::default(), &short_circuit_cache)
        }
    }
}

#[cfg(test)]
mod random_tests {
    use crate::cube::{Cube, Facelet};
    use crate::moves::{CanFullMove, nice_write};
    use crate::setup::parse_line;

    use super::*;

    // Exhibits the problem, though I don't know why. This has a solution of length 9.
    const PROBLEM_CHILD: &'static str = "F2 R' F' F2 U2 R2 F R U' R U2 R' L";

    fn do_test(input: &str, ht: HeuristicType, exp_length: usize) {
        let moves = parse_line(input).unwrap();
        let mut start = Cube::make_solved(Facelet::Green, Facelet::White);

        for m in moves {
            start = start.apply_full(m);
        }

        let start = start;

        let solution = optimal_solve(start.clone(), ht);

        println!("Given scramble \"{}\", got solution \"{}\"", input, nice_write(&solution));

        let mut running = start;

        for m in &solution {
            running = running.apply(*m);
        }

        assert!(running.solved(), "Solution should result in a solved cube");
        assert_eq!(solution.len(), exp_length, "Solution should have the right length; expected {} but got {}.", exp_length, solution.len());
    }

    #[test]
    fn test_sample_no_heuristic() {
        do_test(PROBLEM_CHILD, HeuristicType::None, 9);
    }

    #[test]
    fn test_sample_orr_heuristic() {
        do_test(PROBLEM_CHILD, HeuristicType::Orr, 9);
    }

    #[test]
    fn test_sample_pos_heuristic() {
        do_test(PROBLEM_CHILD, HeuristicType::Pos, 9);
    }

    #[test]
    fn test_sample_full_heuristic() {
        do_test(PROBLEM_CHILD, HeuristicType::PosAndOrr, 9);
    }
}
