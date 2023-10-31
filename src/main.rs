use std::time::Instant;

use clap::{Parser, Subcommand};

use crate::cube::{Cube, Facelet};
use crate::full_solve::{optimal_solve, HeuristicType};
use crate::len_bound::compute_len_bound;
use crate::moves::{flipped, nice_write, CanFullMove, FullMove};
use crate::scramble::{
    full_scramble, scramble_cfl, scramble_cfl_oll, scramble_cll, scramble_ofl, scramble_oll,
};

mod cube;
mod full_solve;
mod len_bound;
mod moves;
mod orr_solve;
mod pos_solve;
mod scramble;
mod setup;

fn solve_input(input: &[FullMove]) {
    let cube = Cube::make_solved(Facelet::Green, Facelet::White).apply_many_full(input);

    let start = Instant::now();
    let solution = optimal_solve(cube, HeuristicType::Orr);
    let elapsed = start.elapsed();

    println!(
        "Full solution to input in {} moves:\n{}",
        solution.len(),
        nice_write(&solution)
    );
    println!("Search took {:?}", elapsed);
}

#[derive(Subcommand, Copy, Clone, Debug)]
enum ScrambleKind {
    /// Performs a full scramble. All permutations are possible.
    Full,
    /// Performs an OFL scramble. The bottom layer will be correctly oriented.
    OFL,
    /// Performs a CFL scramble. The bottom layer will be completely solved.
    CFL,
    /// Performs an OLL scramble. The bottom and top layer will be correctly oriented.
    OLL,
    /// Perfectly scrambles a solved cube into a solved state.
    CLL,
    /// Leave the bottom layer completely solved, and the top layer oriented correctly
    // funny naming but it makes clap happy which is all i wanted really
    #[allow(non_camel_case_types)]
    CFL_OFL,
}

#[derive(Subcommand, Debug, Clone)]
enum CubeCommand {
    Solve {
        permutation: String,
    },
    Scramble {
        #[clap(subcommand)]
        kind: ScrambleKind,
    },
    LengthBound,
}

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    cmd: CubeCommand,
}

fn main() -> Result<(), i32> {
    let args = Arguments::parse();

    match args.cmd {
        CubeCommand::Solve { permutation } => {
            let parsed = setup::parse_line(&permutation).map_err(|e| {
                println!("Could not parse token {:?}", e);
                1
            })?;

            solve_input(&parsed);
        }
        CubeCommand::LengthBound => {
            let start = Instant::now();
            let len_bound = compute_len_bound();
            let elapsed = start.elapsed();
            println!(
                "Determined the optimal length bound for the pocket cube to be {}",
                len_bound
            );
            println!("Derivation took {:?}", elapsed);
        }
        CubeCommand::Scramble { kind } => {
            let scramble = match kind {
                ScrambleKind::Full => full_scramble(),
                ScrambleKind::OFL => scramble_ofl(),
                ScrambleKind::CFL => scramble_cfl(),
                ScrambleKind::OLL => scramble_oll(),
                ScrambleKind::CLL => scramble_cll(),
                ScrambleKind::CFL_OFL => scramble_cfl_oll(),
            };

            let start = Instant::now();
            let solution = optimal_solve(scramble, HeuristicType::Orr);
            let elapsed = start.elapsed();

            println!("Full solution to scramble in {} moves", solution.len());
            println!("Search took {:?}", elapsed);

            let steps = flipped(&solution);
            println!("Scramble given by: {}", nice_write(&steps));
        }
    }

    Ok(())
}
