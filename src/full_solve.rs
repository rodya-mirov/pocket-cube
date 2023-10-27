use std::collections::HashMap;
use crate::cube::{Cube, CubeletArrangement};
use crate::moves::{Amt, CanMove, Dir, Move};
use crate::pos_solve::optimal_solve_position;

#[derive(Default)]
struct PosDistCache {
    pos_dist_cache: HashMap<CubeletArrangement, usize>
}

impl PosDistCache {
    fn dist_to_solved(&mut self, arr: CubeletArrangement) -> usize {
        if let Some(dist) = self.pos_dist_cache.get(&arr) {
            return *dist;
        }

        let solution = optimal_solve_position(arr.clone());
        let full_length = solution.len();

        let mut running = arr;
        let mut remaining_length = solution.len();

        self.pos_dist_cache.insert(running.clone(), remaining_length);

        // it is a fact that if the shortest path from A to B is P, and P passes through C,
        // then the remaining path from C to B is also the shortest path from C to B
        // so this saves us running all 8! = 40320 possible position configurations, which I guess
        // is something
        for m in solution {
            running = running.apply(m);
            remaining_length -= 1;

            self.pos_dist_cache.insert(running.clone(), remaining_length);
        }

        full_length
    }
}

/// Gives an estimated number of moves remaining to solve the cube.
/// Note this must be a LOWER BOUND; that is, it can guess too low (e.g. 0) but cannot be allowed
/// to guess too HIGH (e.g. if there is a solution of length 3, and this gives 4, the algorithm
/// will break)
fn cost_heuristic(arr: CubeletArrangement, cache: &mut PosDistCache) -> usize {
    cache.dist_to_solved(arr)
}

enum SolveResult {
    // found a solution
    Success,
    // failed to find a solution; best pruned node available was [val]
    Failed(usize),
}

// if running is None, gives next; otherwise gives the min of the two values
fn safe_min(running: Option<usize> , next: usize) -> usize {
    match running {
        None => next,
        Some(old) => old.min(next),
    }
}

pub fn optimal_solve(cube: Cube) -> Vec<Move> {
    if cube.solved() {
        return vec![];
    }

    fn solve(cube: Cube, arr: CubeletArrangement, cache: &mut PosDistCache, running: &mut Vec<Move>, max_cost: usize) -> SolveResult {
        if cube.solved() {
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
                    return SolveResult::Success
                }

                let next_arr = arr.clone().apply(m);

                let est_cost = running.len() + cost_heuristic(next_arr.clone(), cache);

                if est_cost <= max_cost {
                    // if we have enough gas to get to the next node, try it out
                    let iterate_result = solve(next_cube, next_arr, cache, running, max_cost);

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

        let next_node_cost = best_failure.expect("Should have found a failure, since there was no success");
        SolveResult::Failed(next_node_cost)
    }


    let mut pos_dist_cache  = PosDistCache::default();



    let pos_arr = cube.clone().make_pos_arr_from_dlb();

    let mut starting_fuel = cost_heuristic(pos_arr.clone(), &mut pos_dist_cache);

    const MAX_FUEL: usize = 13; // supposedly pocket cube is solvable in 11 moves, this is me being suspicious

    while starting_fuel < MAX_FUEL {
        let mut running = Vec::new();

        let sr = solve(cube.clone(), pos_arr.clone(), &mut pos_dist_cache, &mut running, starting_fuel);

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
