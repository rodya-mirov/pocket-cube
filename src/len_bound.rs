//! Functionality for computing the optimal-path-bound for the pocket cube (sometimes called
//! "God's number").

use std::collections::VecDeque;
use std::time::Instant;

use crate::cube::{
    Cube, CubeletOrientation, CubeletOrientationArrangement, CubeletPos,
    CubeletPositionArrangement, Facelet, ALL_CUBIES,
};
use crate::full_solve::{
    optimal_solve_heuristic, FullHeuristic, Heuristic, ShortCircuitCache, SimpleShortCircuitCache,
};
use crate::scramble::put_cubie;

pub fn compute_len_bound() -> usize {
    // basically we're going to iterate through every meaningfully different setup
    // and compute their optimal solution length

    println!("By symmetry, we can assume the DLB corner is white/blue/red, with white on bottom");

    // You can change this heuristic type if you want; if you cut it down, it will correctly
    // save (almost all of the) heuristic pre-computation time
    let mut heuristic = FullHeuristic::default();

    load_orr_heuristic(&mut heuristic);
    load_pos_heuristic(&mut heuristic);

    let mut short_circuit_cache = SimpleShortCircuitCache::default();

    // Note: we know the front/top goal facelets because DLB is fixed
    // Basically this means we'll precompute everything of length up to 9 (which takes about
    // 80% of the allotted time) ...
    let cache_start = Instant::now();
    short_circuit_cache.load_with_depth(9, Facelet::Green, Facelet::Yellow);

    println!(
        "Computed relevant solutions up to depth {} in {:?}",
        9,
        cache_start.elapsed()
    );

    println!(
        "Short circuit cache has {} unique patterns in it",
        short_circuit_cache.cache_size()
    );

    // ... then IDA* every possible combination, short-circuiting as soon as we hit something
    // of accessibility 9 or less.
    try_combinations(&mut heuristic, &short_circuit_cache)

    // (experimentally, 9 was the sweet spot between spending your whole time in the cache, and
    // spending too long per combination)
}

// TODO: to parallelize we need to refactor to allow the immutable reference, pass on that for now
fn try_combinations<H: Heuristic, S: ShortCircuitCache>(h: &mut H, s: &S) -> usize {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::White);

    let mut all_cubies: VecDeque<[Facelet; 3]> = ALL_CUBIES.clone().into_iter().collect();

    let dlb = all_cubies.pop_front().unwrap();

    // first cubie is correct and oriented, by symmetry
    put_cubie(&mut my_cube, 0, dlb, 0);

    let start_time = Instant::now();

    fn recursive_walk<H: Heuristic, S: ShortCircuitCache>(
        cube: &mut Cube,
        remaining_cubelets: &mut VecDeque<[Facelet; 3]>,
        h: &mut H,
        s: &S,
        start: &Instant,
    ) -> usize {
        let num_trials = remaining_cubelets.len();
        let next_pos = 8 - num_trials;

        if next_pos < 3 {
            println!(
                "Walking; next pos is {} (elapsed {:?})",
                next_pos,
                start.elapsed()
            );
        }

        let mut running_max = 0;

        for _ in 0..num_trials {
            let next_cubelet = remaining_cubelets.pop_front().unwrap();

            for orr in 0..3 {
                put_cubie(cube, next_pos as i32, next_cubelet.clone(), orr);
                if next_pos == 7 {
                    if !cube.clone().make_orr_arr_from_dlb().is_solvable() {
                        continue;
                    }
                    let len = optimal_solve_heuristic(cube.clone(), h, s).len();
                    running_max = running_max.max(len);
                } else {
                    let worst = recursive_walk(cube, remaining_cubelets, h, s, start);
                    running_max = running_max.max(worst);
                }
            }

            remaining_cubelets.push_back(next_cubelet);
        }

        running_max
    }

    recursive_walk(&mut my_cube, &mut all_cubies, h, s, &start_time)
}

fn load_pos_heuristic<H: Heuristic>(h: &mut H) {
    let start = Instant::now();

    let mut pos = CubeletPositionArrangement::make_solved();
    // orr doesn't matter but method signature demands it
    let orr = CubeletOrientationArrangement::make_solved();

    pos.dlb = CubeletPos::DLB;

    let considered: usize = 7 * 6 * 5 * 4 * 3 * 2;

    fn recursive_walk<H: Heuristic>(
        pos: &mut CubeletPositionArrangement,
        orr: &CubeletOrientationArrangement,
        next_pos: usize,
        h: &mut H,
        remaining_positions: &mut VecDeque<usize>,
    ) {
        if next_pos == 7 {
            put_pos(pos, next_pos, remaining_positions[0]);
            h.estimated_remaining_cost(pos.clone(), orr.clone());
        } else {
            let num_trials = remaining_positions.len();

            for _ in 0..num_trials {
                let next_trial = remaining_positions.pop_front().unwrap();
                put_pos(pos, next_pos, next_trial);
                recursive_walk(pos, orr, next_pos + 1, h, remaining_positions);
                remaining_positions.push_back(next_trial);
            }
        }
    }

    // first cube goes in in the right place, which is fine by symmetry
    let mut remaining: VecDeque<usize> = (1..8).collect();

    put_pos(&mut pos, 0, 0);

    recursive_walk(&mut pos, &orr, 1, h, &mut remaining);

    let elapsed = start.elapsed();

    println!(
        "Computed position solutions (DLB fixed) for {} unique position permutations in {:?}",
        considered, elapsed
    );
}

fn put_pos(pos_arr: &mut CubeletPositionArrangement, put_pos_ind: usize, val_pos_ind: usize) {
    let orr = match val_pos_ind {
        0 => CubeletPos::DLB,
        1 => CubeletPos::DLF,
        2 => CubeletPos::DRF,
        3 => CubeletPos::DRB,
        4 => CubeletPos::ULB,
        5 => CubeletPos::ULF,
        6 => CubeletPos::URF,
        7 => CubeletPos::URB,
        other => panic!("Bad position index: {}", other),
    };

    match put_pos_ind {
        0 => pos_arr.dlb = orr,
        1 => pos_arr.dlf = orr,
        2 => pos_arr.drf = orr,
        3 => pos_arr.drb = orr,
        4 => pos_arr.ulb = orr,
        5 => pos_arr.ulf = orr,
        6 => pos_arr.urf = orr,
        7 => pos_arr.urb = orr,
        other => panic!("Bad pos index {}", other),
    }
}

fn load_orr_heuristic<H: Heuristic>(h: &mut H) {
    let start = Instant::now();

    // pos doesn't matter but method signature demands it
    let pos = CubeletPositionArrangement::make_solved();
    let mut orr = CubeletOrientationArrangement::make_solved();

    orr.dlb = CubeletOrientation::OK;

    let considered: usize = 3_usize.pow(6);

    fn recursive_walk<H: Heuristic>(
        orr: &mut CubeletOrientationArrangement,
        pos: &CubeletPositionArrangement,
        next_pos: usize,
        h: &mut H,
    ) {
        if next_pos < 7 {
            for orr_ind in 0..3 {
                put_orr(orr, next_pos, orr_ind);
                recursive_walk(orr, pos, next_pos + 1, h);
            }
        } else {
            for orr_ind in 0..3 {
                put_orr(orr, next_pos, orr_ind);
                if orr.is_solvable() {
                    h.estimated_remaining_cost(pos.clone(), orr.clone());
                }
            }
        }
    }

    // first cube goes in oriented, which is fine by symmetry
    put_orr(&mut orr, 0, 0);

    recursive_walk(&mut orr, &pos, 1, h);

    let elapsed = start.elapsed();

    println!(
        "Computed orientation solutions (DLB fixed) for {} unique orientation permutations in {:?}",
        considered, elapsed
    );
}

fn put_orr(orr_arr: &mut CubeletOrientationArrangement, pos_index: usize, orr_ind: usize) {
    let orr = match orr_ind {
        0 => CubeletOrientation::OK,
        1 => CubeletOrientation::CW,
        2 => CubeletOrientation::CCW,
        other => panic!("Bad orientation index: {}", other),
    };

    match pos_index {
        0 => orr_arr.dlb = orr,
        1 => orr_arr.dlf = orr,
        2 => orr_arr.drf = orr,
        3 => orr_arr.drb = orr,
        4 => orr_arr.ulb = orr,
        5 => orr_arr.ulf = orr,
        6 => orr_arr.urf = orr,
        7 => orr_arr.urb = orr,
        other => panic!("Bad pos index {}", other),
    }
}
