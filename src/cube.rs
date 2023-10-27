// A list, in no particular order, of all cubies
// Each cubie is in "order" clockwise, but without a set start cubie; that is, YBO is BOY is OYB,
// but OBY is not equivalent and is not a cubie because that color combination does not occur on
// the pocket cube in that order.
const ALL_CUBIES: [[Facelet; 3]; 8] = [
    // Anything with yellow ...
    [Facelet::Yellow, Facelet::Orange, Facelet::Green],
    [Facelet::Yellow, Facelet::Green, Facelet::Red],
    [Facelet::Yellow, Facelet::Red, Facelet::Blue],
    [Facelet::Yellow, Facelet::Blue, Facelet::Orange],
    // Anything with white ...
    [Facelet::White, Facelet::Blue, Facelet::Red],
    [Facelet::White, Facelet::Red, Facelet::Green],
    [Facelet::White, Facelet::Green, Facelet::Orange],
    [Facelet::White, Facelet::Orange, Facelet::Blue],
];

// Note this is NOT Copy, not because it can't be (it definitely can), but because this makes the borrow
// checker ensure we are actually permuting when we're supposed to be permuting
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Facelet {
    Yellow,
    Red,
    White,
    Orange,
    Blue,
    Green,
}

impl Facelet {
    fn opposite(&self) -> Self {
        match self {
            Facelet::Yellow => Facelet::White,
            Facelet::White => Facelet::Yellow,
            Facelet::Red => Facelet::Orange,
            Facelet::Orange => Facelet::Red,
            Facelet::Blue => Facelet::Green,
            Facelet::Green => Facelet::Blue,
        }
    }
}

impl TryFrom<char> for Facelet {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'Y' | 'y' => Ok(Facelet::Yellow),
            'R' | 'r' => Ok(Facelet::Red),
            'W' | 'w' => Ok(Facelet::White),
            'O' | 'o' => Ok(Facelet::Orange),
            'B' | 'b' => Ok(Facelet::Blue),
            'G' | 'g' => Ok(Facelet::Green),
            other => Err(other),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UDFace {
    bl: Facelet,
    br: Facelet,
    fl: Facelet,
    fr: Facelet,
}

impl UDFace {
    pub fn solved(&self) -> bool {
        self.bl == self.br && self.bl == self.fl && self.bl == self.fr
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FBFace {
    ul: Facelet,
    ur: Facelet,
    dl: Facelet,
    dr: Facelet,
}

impl FBFace {
    pub fn solved(&self) -> bool {
        self.ul == self.ur && self.ul == self.dl && self.ul == self.dr
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LRFace {
    ub: Facelet,
    uf: Facelet,
    db: Facelet,
    df: Facelet,
}

impl LRFace {
    pub fn solved(&self) -> bool {
        self.ub == self.uf && self.ub == self.db && self.ub == self.df
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Cube {
    u: UDFace,
    d: UDFace,
    r: LRFace,
    l: LRFace,
    f: FBFace,
    b: FBFace,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeletArrangement {
    // basically a map from "place on the cube" to "cubelet whose desired place is ..."
    // or; "self.ulb == URF" means "the cubelet which is at ULB needs to be at URF"
    ulf: CubeletPos,
    ulb: CubeletPos,
    urf: CubeletPos,
    urb: CubeletPos,
    dlf: CubeletPos,
    dlb: CubeletPos,
    drf: CubeletPos,
    drb: CubeletPos,
}

impl CubeletArrangement {
    #[inline(always)]
    fn make_solved() -> Self {
        use CubeletPos::*;

        CubeletArrangement {
            ulf: ULF,
            ulb: ULB,
            urf: URF,
            urb: URB,
            dlf: DLF,
            dlb: DLB,
            drf: DRF,
            drb: DRB,
        }
    }

    #[inline(always)]
    pub fn is_solved(&self) -> bool {
        // should optimize to 8 equality checks pretty readily
        self == &CubeletArrangement::make_solved()
    }

    // note: the position-ness is assuming we fix the DLB cubelet, which we can do, I guess
    //       it's not really canonically defined
    // we COULD define the d/l/b moves but it would basically be: whatever moves into the DLB
    // position is now the root of the thing, reevaluate everything according to that; and to do
    // that we do need to also pick an orientation of the cubelet, it's a whole thing, it's awful
    // so instead we're just gonna not have dlb moves, at all; then everything is fine

    #[inline(always)]
    pub fn r(self) -> Self {
        let Self {
            ulf,
            urf,
            ulb,
            urb,
            dlf,
            dlb,
            drf,
            drb,
        } = self;

        Self {
            ulf,
            ulb,
            dlf,
            dlb,
            urf: drf,
            urb: urf,
            drf: drb,
            drb: urb,
        }
    }

    #[inline(always)]
    pub fn r_two(self) -> Self {
        self.r().r()
    }

    #[inline(always)]
    pub fn r_rev(self) -> Self {
        self.r().r().r()
    }

    #[inline(always)]
    pub fn u(self) -> Self {
        let Self {
            ulf,
            urf,
            ulb,
            urb,
            dlf,
            dlb,
            drf,
            drb,
        } = self;

        Self {
            dlf,
            drf,
            dlb,
            drb,
            ulf: urf,
            urf: urb,
            urb: ulb,
            ulb: ulf,
        }
    }

    #[inline(always)]
    pub fn u_two(self) -> Self {
        self.u().u()
    }

    #[inline(always)]
    pub fn u_rev(self) -> Self {
        self.u().u().u()
    }

    #[inline(always)]
    pub fn f(self) -> Self {
        let Self {
            ulf,
            urf,
            ulb,
            urb,
            dlf,
            dlb,
            drf,
            drb,
        } = self;

        Self {
            ulb,
            urb,
            dlb,
            drb,
            ulf: dlf,
            dlf: drf,
            drf: urf,
            urf: ulf,
        }
    }

    #[inline(always)]
    pub fn f_two(self) -> Self {
        self.f().f()
    }

    #[inline(always)]
    pub fn f_rev(self) -> Self {
        self.f().f().f()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DesiredFaces {
    u: Facelet,
    d: Facelet,
    r: Facelet,
    l: Facelet,
    f: Facelet,
    b: Facelet,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CubeletPos {
    ULF,
    ULB,
    URF,
    URB,
    DLF,
    DLB,
    DRF,
    DRB,
}

impl CubeletPos {
    fn from_pos(u: bool, r: bool, f: bool) -> CubeletPos {
        use CubeletPos::*;

        match (u, r, f) {
            (true, true, true) => URF,
            (true, true, false) => URB,
            (true, false, true) => ULF,
            (true, false, false) => ULB,
            (false, true, true) => DRF,
            (false, true, false) => DRB,
            (false, false, true) => DLF,
            (false, false, false) => DLB,
        }
    }

    // TODO: remove this? not sure it's needed
    #[allow(unused)]
    fn u(&self) -> bool {
        match self {
            CubeletPos::ULF => true,
            CubeletPos::ULB => true,
            CubeletPos::URF => true,
            CubeletPos::URB => true,
            CubeletPos::DLF => false,
            CubeletPos::DLB => false,
            CubeletPos::DRF => false,
            CubeletPos::DRB => false,
        }
    }

    // TODO: remove this? not sure it's needed
    #[allow(unused)]
    fn r(&self) -> bool {
        match self {
            CubeletPos::ULF => false,
            CubeletPos::ULB => false,
            CubeletPos::URF => true,
            CubeletPos::URB => true,
            CubeletPos::DLF => false,
            CubeletPos::DLB => false,
            CubeletPos::DRF => true,
            CubeletPos::DRB => true,
        }
    }

    // TODO: remove this? not sure it's needed
    #[allow(unused)]
    fn f(&self) -> bool {
        match self {
            CubeletPos::ULF => true,
            CubeletPos::ULB => false,
            CubeletPos::URF => true,
            CubeletPos::URB => false,
            CubeletPos::DLF => true,
            CubeletPos::DLB => false,
            CubeletPos::DRF => true,
            CubeletPos::DRB => false,
        }
    }
}

/// Returns the "next" color for a (corner) cubie, in clockwise order, starting from a, to b,
/// to (the return value).
fn next_color(a: Facelet, b: Facelet) -> Facelet {
    if a.opposite() == b || a == b {
        panic!(
            "The colors {:?} and {:?} do not appear on a corner together",
            a, b
        )
    }

    for cubie in ALL_CUBIES {
        for i in 0..3 {
            if a == cubie[i] {
                let next = (i + 1) % 3;
                if b == cubie[next] {
                    let last = (next + 1) % 3;
                    return cubie[last].clone();
                }
            }
        }
    }

    unreachable!("Given the facelets {:?} and {:?} which are not equal or opposites, we should have found a third facelet, but didn't", a, b)
}

fn make_pos_from_dlb(
    desired_faces: &DesiredFaces,
    a: Facelet,
    b: Facelet,
    c: Facelet,
) -> CubeletPos {
    let mut u = None;
    let mut r = None;
    let mut f = None;

    for facelet in [a, b, c] {
        if facelet == desired_faces.u {
            u = Some(true);
        }
        if facelet == desired_faces.d {
            u = Some(false);
        }
        if facelet == desired_faces.r {
            r = Some(true);
        }
        if facelet == desired_faces.l {
            r = Some(false);
        }
        if facelet == desired_faces.f {
            f = Some(true);
        }
        if facelet == desired_faces.b {
            f = Some(false);
        }
    }

    CubeletPos::from_pos(
        u.expect("Should find a u or d match"),
        r.expect("Should find a r or l match"),
        f.expect("Should find a f or b match"),
    )
}

impl Cube {
    pub fn make_solved(front_color: Facelet, up_color: Facelet) -> Self {
        let back_color = front_color.opposite();
        let down_color = up_color.opposite();

        let right_color = next_color(front_color.clone(), up_color.clone());
        let left_color = right_color.opposite();

        Self {
            u: UDFace {
                bl: up_color.clone(),
                br: up_color.clone(),
                fl: up_color.clone(),
                fr: up_color,
            },
            d: UDFace {
                bl: down_color.clone(),
                br: down_color.clone(),
                fl: down_color.clone(),
                fr: down_color,
            },
            r: LRFace {
                ub: right_color.clone(),
                uf: right_color.clone(),
                db: right_color.clone(),
                df: right_color,
            },
            l: LRFace {
                ub: left_color.clone(),
                uf: left_color.clone(),
                db: left_color.clone(),
                df: left_color,
            },
            f: FBFace {
                ul: front_color.clone(),
                ur: front_color.clone(),
                dl: front_color.clone(),
                dr: front_color,
            },
            b: FBFace {
                ul: back_color.clone(),
                ur: back_color.clone(),
                dl: back_color.clone(),
                dr: back_color,
            },
        }
    }

    fn make_desired_from_dlb(&self) -> DesiredFaces {
        let l = self.l.db.clone();
        let d = self.d.bl.clone();
        let b = self.b.dl.clone();

        let r = l.opposite();
        let u = d.opposite();
        let f = b.opposite();

        DesiredFaces { l, d, b, r, u, f }
    }

    pub fn make_pos_arr_from_dlb(self) -> CubeletArrangement {
        let des = self.make_desired_from_dlb();

        let Self { u, d, r, l, f, b } = self;

        let dlb = make_pos_from_dlb(&des, d.bl, l.db, b.dl); // better be DLB
        let drb = make_pos_from_dlb(&des, d.br, r.db, b.dr);
        let dlf = make_pos_from_dlb(&des, d.fl, l.df, f.dl);
        let drf = make_pos_from_dlb(&des, d.fr, r.df, f.dr);

        let ulb = make_pos_from_dlb(&des, u.bl, l.ub, b.ul);
        let urb = make_pos_from_dlb(&des, u.br, r.ub, b.ur);
        let ulf = make_pos_from_dlb(&des, u.fl, l.uf, f.ul);
        let urf = make_pos_from_dlb(&des, u.fr, r.uf, f.ur);

        CubeletArrangement {
            dlb,
            drb,
            dlf,
            drf,
            ulb,
            urb,
            ulf,
            urf,
        }
    }

    #[inline(always)]
    pub fn solved(&self) -> bool {
        self.u.solved()
            && self.d.solved()
            && self.r.solved()
            && self.l.solved()
            && self.f.solved()
            && self.b.solved()
    }

    /// Get the result of the L action
    #[inline(always)]
    pub fn left(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            r,
            l: LRFace {
                ub: l.db,
                uf: l.ub,
                db: l.df,
                df: l.uf,
            },
            u: UDFace {
                bl: b.dl,
                br: u.br,
                fl: b.ul,
                fr: u.fr,
            },
            d: UDFace {
                bl: f.dl,
                br: d.br,
                fl: f.ul,
                fr: d.fr,
            },
            f: FBFace {
                ul: u.bl,
                ur: f.ur,
                dl: u.fl,
                dr: f.dr,
            },
            b: FBFace {
                ul: d.bl,
                ur: b.ur,
                dl: d.fl,
                dr: b.dr,
            },
        }
    }

    #[inline(always)]
    pub fn left_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.left().left()
    }

    #[inline(always)]
    pub fn left_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.left().left().left()
    }

    #[inline(always)]
    pub fn right(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            l,
            r: LRFace {
                ub: r.uf,
                uf: r.df,
                db: r.ub,
                df: r.db,
            },
            u: UDFace {
                bl: u.bl,
                br: f.ur,
                fl: u.fl,
                fr: f.dr,
            },
            d: UDFace {
                bl: d.bl,
                br: b.ur,
                fl: d.fl,
                fr: b.dr,
            },
            f: FBFace {
                ul: f.ul,
                ur: d.fr,
                dl: f.dl,
                dr: d.br,
            },
            b: FBFace {
                ul: b.ul,
                ur: u.fr,
                dl: b.dl,
                dr: u.br,
            },
        }
    }

    #[inline(always)]
    pub fn right_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.right().right()
    }

    #[inline(always)]
    pub fn right_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.right().right().right()
    }

    #[inline(always)]
    pub fn up(self) -> Self {
        let Self { u, f, b, r, l, d } = self;

        Self {
            d,
            u: UDFace {
                bl: u.fl,
                br: u.bl,
                fl: u.fr,
                fr: u.br,
            },
            r: LRFace {
                db: r.db,
                df: r.df,
                ub: b.ul,
                uf: b.ur,
            },
            l: LRFace {
                db: l.db,
                df: l.df,
                ub: f.ul,
                uf: f.ur,
            },
            f: FBFace {
                dl: f.dl,
                dr: f.dr,
                ul: r.uf,
                ur: r.ub,
            },
            b: FBFace {
                dl: b.dl,
                dr: b.dr,
                ul: l.uf,
                ur: l.ub,
            },
        }
    }

    #[inline(always)]
    pub fn up_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.up().up()
    }

    #[inline(always)]
    pub fn up_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.up().up().up()
    }

    #[inline(always)]
    pub fn front(self) -> Self {
        let Self { u, f, b, r, l, d } = self;

        Self {
            b,
            f: FBFace {
                ul: f.dl,
                ur: f.ul,
                dl: f.dr,
                dr: f.ur,
            },
            u: UDFace {
                bl: u.bl,
                br: u.br,
                fl: l.df,
                fr: l.uf,
            },
            d: UDFace {
                bl: d.bl,
                br: d.br,
                fl: r.df,
                fr: r.uf,
            },
            r: LRFace {
                uf: u.fl,
                df: u.fr,
                ub: r.ub,
                db: r.db,
            },
            l: LRFace {
                uf: d.fl,
                df: d.fr,
                ub: l.ub,
                db: l.db,
            },
        }
    }

    #[inline(always)]
    pub fn front_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.front().front()
    }

    #[inline(always)]
    pub fn front_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.front().front().front()
    }

    #[inline(always)]
    pub fn down(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            u,
            d: UDFace {
                fl: d.bl,
                fr: d.fl,
                bl: d.br,
                br: d.fr,
            },
            f: FBFace {
                ul: f.ul,
                ur: f.ur,
                dl: l.db,
                dr: l.df,
            },
            b: FBFace {
                ul: b.ul,
                ur: b.ur,
                dl: r.db,
                dr: r.df,
            },
            r: LRFace {
                uf: r.uf,
                ub: r.ub,
                df: f.dl,
                db: f.dr,
            },
            l: LRFace {
                uf: l.uf,
                ub: l.ub,
                df: b.dl,
                db: b.dr,
            },
        }
    }

    #[inline(always)]
    pub fn down_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.down().down()
    }

    #[inline(always)]
    pub fn down_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.down().down().down()
    }

    #[inline(always)]
    pub fn back(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            f,
            b: FBFace {
                ur: b.dr,
                ul: b.ur,
                dr: b.dl,
                dl: b.ul,
            },
            u: UDFace {
                fl: u.fl,
                fr: u.fr,
                bl: r.ub,
                br: r.db,
            },
            d: UDFace {
                fl: d.fl,
                fr: d.fr,
                bl: l.ub,
                br: l.db,
            },
            r: LRFace {
                uf: r.uf,
                df: r.df,
                ub: d.br,
                db: d.bl,
            },
            l: LRFace {
                uf: l.uf,
                df: l.df,
                ub: u.br,
                db: u.bl,
            },
        }
    }

    #[inline(always)]
    pub fn back_two(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.back().back()
    }

    #[inline(always)]
    pub fn back_rev(self) -> Self {
        // TODO PERF: I'm pretty sure llvm will consistently optimize this into the correct direct code but
        //      I should probably check
        self.back().back().back()
    }
}

#[cfg(test)]
mod invariants_tests {
    use std::panic;

    use super::*;

    fn should_panic<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let r = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        assert!(r.is_err());
    }

    fn facelets() -> [Facelet; 6] {
        [
            Facelet::Blue,
            Facelet::Orange,
            Facelet::White,
            Facelet::Green,
            Facelet::Red,
            Facelet::Yellow,
        ]
    }

    #[test]
    fn opp_test() {
        assert_eq!(Facelet::Yellow.opposite(), Facelet::White);
        assert_eq!(Facelet::White.opposite(), Facelet::Yellow);
        assert_eq!(Facelet::Green.opposite(), Facelet::Blue);
        assert_eq!(Facelet::Blue.opposite(), Facelet::Green);
        assert_eq!(Facelet::Red.opposite(), Facelet::Orange);
        assert_eq!(Facelet::Orange.opposite(), Facelet::Red);

        for v in facelets() {
            assert_eq!(v.opposite().opposite(), v);
        }
    }

    #[test]
    fn corner_exists_test() {
        for a in facelets() {
            for b in facelets() {
                if a != b && a.opposite() != b {
                    // basically just assert we get _something_
                    let c = next_color(a.clone(), b.clone());
                    assert_ne!(a, c);
                    assert_ne!(b, c);
                } else {
                    // otherwise, we do want to make sure it panics, there should be nothing there
                    should_panic(|| next_color(a.clone(), b.clone()));
                }
            }
        }
    }

    #[test]
    fn solved_cube_exists() {
        for a in facelets() {
            for b in facelets() {
                if a != b && a.opposite() != b {
                    // basically just assert we get _something_
                    let _ = Cube::make_solved(a.clone(), b.clone());
                } else {
                    // otherwise, we do want to make sure it panics, there should be nothing there
                    should_panic(|| Cube::make_solved(a.clone(), b.clone()));
                }
            }
        }
    }

    fn assert_period<F: Fn(Cube) -> Cube>(f: F, period: usize, name: &str) {
        let start = Cube::make_solved(Facelet::Green, Facelet::White);

        let mut running = start.clone();
        let mut i = 0;

        running = f(running);
        i += 1;

        while i < period {
            assert_ne!(
                running, start,
                "Should not reach the start state after {:?} iterations of {}",
                i, name
            );
            running = f(running);
            i += 1;
        }

        assert_eq!(
            running, start,
            "Should reach the start state after {:?} iterations of {}",
            period, name
        );
    }

    #[test]
    fn simple_assertions_left() {
        let start = Cube::make_solved(Facelet::Green, Facelet::White);
        let next = start.left();

        let f = next.f;

        assert_eq!(
            f,
            FBFace {
                ul: Facelet::White,
                ur: Facelet::Green,
                dl: Facelet::White,
                dr: Facelet::Green
            }
        );
    }

    #[test]
    fn simple_assertions_right() {
        let start = Cube::make_solved(Facelet::Green, Facelet::White);
        let next = start.right();

        let f = next.f;

        assert_eq!(
            f,
            FBFace {
                ul: Facelet::Green,
                ur: Facelet::Yellow,
                dl: Facelet::Green,
                dr: Facelet::Yellow
            }
        );
    }

    #[test]
    fn simple_assertions_mid() {
        let start = Cube::make_solved(Facelet::Yellow, Facelet::Orange);

        let next = start.clone().back().down().right();

        let f = next.f;

        assert_eq!(
            f,
            FBFace {
                ul: Facelet::Yellow,
                ur: Facelet::Red,
                dl: Facelet::Orange,
                dr: Facelet::Red
            }
        );
    }

    #[test]
    fn simple_assertions_complex() {
        let start = Cube::make_solved(Facelet::Yellow, Facelet::Orange);

        let next = start.clone().back().down().right().front().up().left();

        let f = next.f;

        assert_eq!(
            f,
            FBFace {
                ul: Facelet::White,
                ur: Facelet::Green,
                dl: Facelet::Blue,
                dr: Facelet::Red
            }
        );
    }

    #[test]
    fn simple_rotations() {
        let start = || Cube::make_solved(Facelet::Red, Facelet::Yellow);

        assert_ne!(
            Cube::make_solved(Facelet::Green, Facelet::White),
            Cube::make_solved(Facelet::Green, Facelet::White).left()
        );

        assert_period(|cube| cube.left(), 4, "L");
        assert_period(|cube| cube.left_two(), 2, "L2");
        assert_period(|cube| cube.left_rev(), 4, "L'");

        assert_period(|cube| cube.right(), 4, "R");
        assert_period(|cube| cube.right_two(), 2, "R2");
        assert_period(|cube| cube.right_rev(), 4, "R'");

        assert_period(|cube| cube.up(), 4, "U");
        assert_period(|cube| cube.up_two(), 2, "U2");
        assert_period(|cube| cube.up_rev(), 4, "U'");

        assert_period(|cube| cube.down(), 4, "D");
        assert_period(|cube| cube.down_two(), 2, "D2");
        assert_period(|cube| cube.down_rev(), 4, "D'");

        assert_period(|cube| cube.back(), 4, "B");
        assert_period(|cube| cube.back_two(), 2, "B2");
        assert_period(|cube| cube.back_rev(), 4, "B'");

        assert_period(|cube| cube.front(), 4, "F");
        assert_period(|cube| cube.front_two(), 2, "F2");
        assert_period(|cube| cube.front_rev(), 4, "F'");
    }
}
