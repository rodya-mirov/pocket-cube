use std::collections::HashMap;

use crate::cube::{Cube, CubeletOrientationArrangement, CubeletPositionArrangement, Facelet};
use crate::moves::{Amt, CanMove, Dir, Move};
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
    fn learn_solution(&mut self, cube: Cube, solution: Vec<Move>);
    fn known_solution(&self, cube: &Cube) -> Option<Vec<Move>>;

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
                None => cache.learn_solution(cube.clone(), running.clone()),
                Some(existing) => {
                    if running.len() >= existing.len() {
                        return;
                    } else {
                        cache.learn_solution(cube.clone(), running.clone())
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
}

impl SimpleShortCircuitCache {
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl ShortCircuitCache for SimpleShortCircuitCache {
    fn learn_solution(&mut self, cube: Cube, solution: Vec<Move>) {
        self.cache.insert(cube, solution);
    }

    fn known_solution(&self, cube: &Cube) -> Option<Vec<Move>> {
        self.cache.get(&cube).cloned()
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

enum SolveResult {
    // found a solution
    Success,
    // failed to find a solution; best pruned node available was [val]
    Failed(usize),
}

// if running is None, gives next; otherwise gives the min of the two values
fn safe_min(running: Option<usize>, next: usize) -> usize {
    match running {
        None => next,
        Some(old) => old.min(next),
    }
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
        }

        if let Some(known) = short_circuit_cache.known_solution(&cube) {
            for m in known {
                running.push(m);
            }
            return SolveResult::Success;
        }

        let mut best_failure = None;

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

                let est_cost = running.len()
                    + heuristic
                        .estimated_remaining_cost(next_pos_arr.clone(), next_orr_arr.clone());

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
                        SolveResult::Failed(best_pruned) => {
                            best_failure = Some(safe_min(best_failure, best_pruned));
                        }
                    }
                } else {
                    // if we're out of gas, note how much fuel we would have needed to look at
                    // the next node, and move on
                    best_failure = Some(safe_min(best_failure, est_cost));
                }

                running.pop();
            }
        }

        let next_node_cost =
            best_failure.expect("Should have found a failure, since there was no success");
        SolveResult::Failed(next_node_cost)
    }

    fn solve_with_heuristic<H: Heuristic, S: ShortCircuitCache>(
        cube: Cube,
        heuristic: &mut H,
        max_fuel: usize,
        short_circuit_cache: &S,
    ) -> Vec<Move> {
        let pos_arr = cube.clone().make_pos_arr_from_dlb();
        let orr_arr = cube.clone().make_orr_arr_from_dlb();

        let mut starting_fuel =
            heuristic.estimated_remaining_cost(pos_arr.clone(), orr_arr.clone());

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

            match sr {
                SolveResult::Success => return running,
                SolveResult::Failed(next_fuel) => {
                    assert!(next_fuel > starting_fuel);
                    starting_fuel = next_fuel;
                }
            }
        }

        unreachable!("Should have found a solution!")
    }

    const MAX_FUEL: usize = 13;

    solve_with_heuristic(cube, heuristic, MAX_FUEL, short_circuit_cache)
}

pub fn optimal_solve(cube: Cube, heuristic_type: HeuristicType) -> Vec<Move> {
    let mut short_circuit_cache = SimpleShortCircuitCache::default();
    let des = cube.clone().make_desired_from_dlb();

    short_circuit_cache.load_with_depth(5, des.f, des.u);

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
