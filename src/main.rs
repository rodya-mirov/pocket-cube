use std::env;
use std::time::Instant;

use crate::cube::{Cube, Facelet};
use crate::full_solve::{optimal_solve, HeuristicType};
use crate::moves::{flipped, nice_write, CanFullMove, FullMove};
use crate::scramble::{scramble_ofl, full_scramble, scramble_cfl};

mod cube;
mod full_solve;
mod moves;
mod orr_solve;
mod pos_solve;
mod scramble;
mod setup;

/*
Terminology we're going to fix:

    CUBE -- the whole cube object itself
    CUBELET / CUBIE -- the individual atomic block that moves around (in a pocket cube, only corner cubies)
        the terms CUBELET and CUBIE will be used interchangeably
    FACE -- one full "side" of the cube, which has four FACELETs on it
    FACELET -- one little tile / sticker on the cube; on a pocket cube, each cubelet has 3 facelets

    POSITION -- the position (not including orientation) of a cubie on the cube; there are eight possible
        positions on the pocket cube, and any cubie can go in any position.

        Relative to a fixed cubie (see below), we can say a particular cubie is in a correct position
        or not.

        Note: the statement "if we fix A, B is in the correct position" is symmetric between A and B;
            that is, one is true iff the other is true

    POSITIONALLY SOLVED -- a cube is positionally solved if every cubie is in a position where, if you
        rotated them individually by whatever means, the puzzle would be solved

        Three equivalents:
        - There is a solution (possibly involving a screwdriver) to the cube which only changes
            orientations of cubelets, and not positions
        - For some cubelet, if we consider it fixed, every other cubelet is in the correct position
        - For _every_ cubelet, if we consider it fixed, every other cubelet is in the correct position

    ORIENTATION -- the "rotation" of a cubelet in its position. There are three possible orientations of
        a cubelet in a given position.

        Relative to a fixed cubie, we can say a cubelet is in the _correct_ orientation if its side facelet
            (that is, the facelet which is on a side FACE) _should_ be on a side FACE. For instance if we
            know the front "should" be green and the top "should" be white, then the side faces are orange
            and red; so a cubie is in correct orientation if its red or orange facelet is actually on a
            side face.

        Relative to a fixed cubie, a cubelet has orientation 1 if it is clockwise-rotated one time from
            the correct orienation. If has orientation 2 if it is counter-clockwise-rotated one time
            from correct (or, equivalently, clockwise-rotated two times).

            Because three turns bring you back "home" we take orienation mod 3 (so e.g. -1 is equivalent
            to 2) and so on.

        Note that all moves maintain the "total orientation" (mod 3) of the cube, and a solved cube has
        total orientation 0; thus any configuration (again, possibly achieved using a screwdriver) whose
        total orientation of something other than zero is unsolveable. This turns out to be the only
        invariant that matters; every screwdriver-reachable configuration which has total orientation zero
        is solveable.

        Note: the statement "if A is fixed, then B has orientation P" is symmetric between A and B.
                (where P could be zero, one, or two)

    ORIENTATIONALLY SOLVED -- a cube is orientationally solved (relative to a fixed cubie; see below)
        if every cubie has orientation zero.

Note that unlike a 3x3 cube, there are no fixed centers; therefore we cannot say a particular
piece is in the "correct" place or not because any solution to the cube has an equivalent solution, of
the same length, which does not move or reorient a given piece. To see this, observe that the R and L
moves result in _literally the same cube_, except you're holding it differently (that is, R and Lx
result in the same cube). Thus you could replace the use of R with Lx; then transform the subsequent moves
to sort of have the x "built in" (basically, the moves they would have been if you didn't do the x),
and you now have a solution of the same length with R taken out.

So in this way we can swap R and L; U and D; and F and B. Thus if your favorite cubie starts on the top,
we can replace every U with D and never move it from the top, and so on.

For me, the easiest moves are R/U/F; therefore, we'll canonically fix the BLD (back/left/down) corner
cubie, which is equivalent to saying we want to find solutions only using R/U/F.

As a nice bonus, once a particular cubie has its position and orientation fixed, everything else
has a canonical correct position and orientation (relative to your fixed cubie). So when we want
to know if a _cubelet_ has the correct position or orientation, we'll do it assuming the BLD
(lower-left-back) cubie is fixed. Then there is a canonical answer.
*/

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

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        let scramble = scramble_cfl();
        println!("I made a scramble!");

        let start = Instant::now();
        let solution = optimal_solve(scramble, HeuristicType::Orr);
        let elapsed = start.elapsed();

        println!("Full solution to scramble in {} moves", solution.len());
        println!("Search took {:?}", elapsed);

        let steps = flipped(&solution);
        println!("Scramble given by: {}", nice_write(&steps));
    } else {
        let input_line: String = args.join(" ");
        let parsed = setup::parse_line(&input_line).map_err(|e| {
            println!("Could not parse token {:?}", e);
            1
        })?;

        solve_input(&parsed);
    }

    Ok(())
}
