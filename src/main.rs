use std::time::Instant;

use clap::{Parser, Subcommand};

use crate::cube::{Cube, Facelet};
use crate::full_solve::{optimal_solve, HeuristicType};
use crate::moves::{flipped, nice_write, CanFullMove, FullMove};
use crate::scramble::{full_scramble, scramble_cfl, scramble_ofl};

mod cube;
mod full_solve;
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
        CubeCommand::Scramble { kind } => {
            let scramble = match kind {
                ScrambleKind::Full => {
                    println!("Doing a full scramble ...");
                    full_scramble()
                }
                ScrambleKind::OFL => {
                    println!("Doing an OFL scramble; first layer should be oriented ...");
                    scramble_ofl()
                }
                ScrambleKind::CFL => {
                    println!("Doing a CFL scramble; first layer should be completely solved ...");
                    scramble_cfl()
                }
                ScrambleKind::OLL => {
                    println!(
                        "Doing an OLL scramble; first and second layer should be oriented ..."
                    );
                    unimplemented!()
                }
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
