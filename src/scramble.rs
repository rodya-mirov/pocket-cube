//! Idea here is to construct a cube which is scrambled, but which can be legally solved

use itertools::Itertools;
use rand::Rng;

use crate::cube::{ALL_CUBIES, Cube, Facelet};

pub fn scramble_ofl() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    let mut rng = rand::thread_rng();

    // take random white cubelets to put into the cube
    put_cubie(&mut my_cube, 0, all_cubies.remove(rng.gen_range(0..4)), 0);
    put_cubie(&mut my_cube, 1, all_cubies.remove(rng.gen_range(0..3)), 0);
    put_cubie(&mut my_cube, 2, all_cubies.remove(rng.gen_range(0..2)), 0);
    put_cubie(&mut my_cube, 3, all_cubies.remove(rng.gen_range(0..1)), 0);

    // then do the yellow cubies at the end
    for i in 4..7 {
        let ind = rng.gen_range(0..all_cubies.len());
        let cubie = all_cubies.remove(ind);

        let rotation = rng.gen_range(0..3);

        put_cubie(&mut my_cube, i, cubie, rotation);
    }

    let cubie = all_cubies.remove(0);

    assert!(all_cubies.is_empty());

    for rotation in 0..3 {
        put_cubie(&mut my_cube, 7, cubie.clone(), rotation);

        if my_cube.clone().make_orr_arr_from_dlb().is_solvable() {
            return my_cube;
        }
    }

    unreachable!("Really should have found a valid orientation for that last cube")
}

pub fn scramble_cfl() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    let mut rng = rand::thread_rng();

    // take random white cubelets to put into the cube
    put_cubie(&mut my_cube, 0, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 1, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 2, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 3, all_cubies.remove(0), 0);

    // then do the yellow cubies at the end
    for i in 4..7 {
        let cubie = all_cubies.remove(rng.gen_range(0..all_cubies.len()));

        let rotation = rng.gen_range(0..3);

        put_cubie(&mut my_cube, i, cubie, rotation);
    }

    let cubie = all_cubies.remove(0);

    assert!(all_cubies.is_empty());

    for rotation in 0..3 {
        put_cubie(&mut my_cube, 7, cubie.clone(), rotation);

        if my_cube.clone().make_orr_arr_from_dlb().is_solvable() {
            return my_cube;
        }
    }

    unreachable!("Really should have found a valid orientation for that last cube")
}

pub fn scramble_oll() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    let mut rng = rand::thread_rng();

    // take random white cubelets to put into the cube
    put_cubie(&mut my_cube, 0, all_cubies.remove(rng.gen_range(0..4)), 0);
    put_cubie(&mut my_cube, 1, all_cubies.remove(rng.gen_range(0..3)), 0);
    put_cubie(&mut my_cube, 2, all_cubies.remove(rng.gen_range(0..2)), 0);
    put_cubie(&mut my_cube, 3, all_cubies.remove(rng.gen_range(0..1)), 0);

    // then do the yellow cubies at the end; we'll scramble the order but leave the orientations
    // fixed. Which is a bit iffy ... ??
    let mut scrambled_cubies = Vec::with_capacity(4);
    for _ in 0..4 {
        scrambled_cubies.push(all_cubies.remove(rng.gen_range(0..all_cubies.len())));
    }

    drop(all_cubies);

    // to control the orientation parity problem, we have to arrange them nicely into the top
    // layer; we scramble in advance, then do this deterministically, so we should have an
    // unbiased distribution
    for perm in (0..4).permutations(4) {
        let mut pos = 4;
        for cubelet_ind in perm {
            let cubie = scrambled_cubies[cubelet_ind].clone();
            put_cubie(&mut my_cube, pos, cubie, 0);
            pos += 1;
        }

        if my_cube.clone().make_orr_arr_from_dlb().is_solvable() {
            return my_cube;
        }
    }

    unreachable!("Really should have found a valid orientation for that last cube")
}

pub fn scramble_cfl_oll() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    let mut rng = rand::thread_rng();

    // take specific white cubelets to put into the cube, so the bottom is completely perfect
    put_cubie(&mut my_cube, 0, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 1, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 2, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 3, all_cubies.remove(0), 0);

    // then do the yellow cubies at the end; we'll scramble the order but leave the orientations
    // fixed. Which is a bit iffy ... ??
    let mut scrambled_cubies = Vec::with_capacity(4);
    for _ in 0..4 {
        scrambled_cubies.push(all_cubies.remove(rng.gen_range(0..all_cubies.len())));
    }

    drop(all_cubies);

    // to control the orientation parity problem, we have to arrange them nicely into the top
    // layer; we scramble in advance, then do this deterministically, so we should have an
    // unbiased distribution
    for perm in (0..4).permutations(4) {
        let mut pos = 4;
        for cubelet_ind in perm {
            let cubie = scrambled_cubies[cubelet_ind].clone();
            put_cubie(&mut my_cube, pos, cubie, 0);
            pos += 1;
        }

        if my_cube.clone().make_orr_arr_from_dlb().is_solvable() {
            return my_cube;
        }
    }

    unreachable!("Really should have found a valid orientation for that last cube")
}

pub fn scramble_cll() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    // take specific white cubelets to put into the cube
    put_cubie(&mut my_cube, 0, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 1, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 2, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 3, all_cubies.remove(0), 0);

    // then do specific yellow cubies at the end
    put_cubie(&mut my_cube, 4, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 5, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 6, all_cubies.remove(0), 0);
    put_cubie(&mut my_cube, 7, all_cubies.remove(0), 0);

    my_cube
}

pub fn full_scramble() -> Cube {
    let mut my_cube = Cube::make_solved(Facelet::Green, Facelet::White);

    let mut all_cubies: Vec<[Facelet; 3]> = ALL_CUBIES.into_iter().collect();

    let mut rng = rand::thread_rng();

    for i in 0..7 {
        let ind = rng.gen_range(0..all_cubies.len());
        let cubie = all_cubies.remove(ind);

        let rotation = rng.gen_range(0..3);

        put_cubie(&mut my_cube, i, cubie, rotation);
    }

    let cubie = all_cubies.remove(0);

    assert!(all_cubies.is_empty());

    for rotation in 0..3 {
        put_cubie(&mut my_cube, 7, cubie.clone(), rotation);

        if my_cube.clone().make_orr_arr_from_dlb().is_solvable() {
            return my_cube;
        }
    }

    unreachable!("Really should have found a valid orientation for that last cube")
}

fn put_cubie(cube: &mut Cube, pos_index: i32, mut cubie: [Facelet; 3], orientation: i32) {
    for _ in 0..orientation {
        cubie.rotate_left(1);
    }

    match pos_index {
        0 => put_dlb(cube, cubie),
        1 => put_dlf(cube, cubie),
        2 => put_drf(cube, cubie),
        3 => put_drb(cube, cubie),
        4 => put_ulb(cube, cubie),
        5 => put_ulf(cube, cubie),
        6 => put_urf(cube, cubie),
        7 => put_urb(cube, cubie),
        _ => unreachable!("Only 8 positions on a pocket cube; got {}", pos_index),
    }
}

fn put_dlb(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the bottom
    cube.d.bl = x;
    cube.b.dl = y;
    cube.l.db = z;
}

fn put_dlf(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the bottom
    cube.d.fl = x;
    cube.l.df = y;
    cube.f.dl = z;
}

fn put_drb(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the bottom
    cube.d.br = x;
    cube.r.db = y;
    cube.b.dr = z;
}

fn put_drf(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the bottom
    cube.d.fr = x;
    cube.f.dr = y;
    cube.r.df = z;
}

fn put_ulb(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the top
    cube.u.bl = x;
    cube.l.ub = y;
    cube.b.ul = z;
}

fn put_urb(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the top
    cube.u.br = x;
    cube.b.ur = y;
    cube.r.ub = z;
}

fn put_ulf(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the top
    cube.u.fl = x;
    cube.f.ul = y;
    cube.l.uf = z;
}

fn put_urf(cube: &mut Cube, cubie: [Facelet; 3]) {
    let [x, y, z] = cubie;

    // these must be put in in clockwise order, starting on the top
    cube.u.fr = x;
    cube.r.uf = y;
    cube.f.ur = z;
}
