//! Idea here is to construct a cube which is scrambled, but which can be legally solved

use rand::Rng;

use crate::cube::{Cube, Facelet, ALL_CUBIES};

pub fn scrambled_cube() -> Cube {
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
        2 => put_drb(cube, cubie),
        3 => put_drf(cube, cubie),
        4 => put_ulb(cube, cubie),
        5 => put_ulf(cube, cubie),
        6 => put_urb(cube, cubie),
        7 => put_urf(cube, cubie),
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
