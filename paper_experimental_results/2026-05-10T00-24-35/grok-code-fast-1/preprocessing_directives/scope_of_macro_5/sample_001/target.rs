use std::cell::RefCell;

thread_local! {
    static RNG_STATE: RefCell<u32> = RefCell::new(2463534242);
}

fn rng_next() -> u32 {
    RNG_STATE.with(|state| {
        let mut x = *state.borrow();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *state.borrow_mut() = x;
        x
    })
}

fn pick_1_9() -> i32 {
    (rng_next() % 9) as i32 + 1
}

fn pick_0_9() -> i32 {
    (rng_next() % 10) as i32
}

macro_rules! gen_xyz_val {
    ($a:expr, $b:expr, $c:expr) => {
        100 * $a + 10 * $b + $c
    };
}

macro_rules! gen_xyz_for_b {
    ($a:expr, $b:expr) => {
        [
            gen_xyz_val!($a, $b, 0),
            gen_xyz_val!($a, $b, 1),
            gen_xyz_val!($a, $b, 2),
            gen_xyz_val!($a, $b, 3),
            gen_xyz_val!($a, $b, 4),
            gen_xyz_val!($a, $b, 5),
            gen_xyz_val!($a, $b, 6),
            gen_xyz_val!($a, $b, 7),
            gen_xyz_val!($a, $b, 8),
            gen_xyz_val!($a, $b, 9),
        ]
    };
}

macro_rules! gen_xyz_for_a {
    ($a:expr) => {
        [
            gen_xyz_for_b!($a, 0),
            gen_xyz_for_b!($a, 1),
            gen_xyz_for_b!($a, 2),
            gen_xyz_for_b!($a, 3),
            gen_xyz_for_b!($a, 4),
            gen_xyz_for_b!($a, 5),
            gen_xyz_for_b!($a, 6),
            gen_xyz_for_b!($a, 7),
            gen_xyz_for_b!($a, 8),
            gen_xyz_for_b!($a, 9),
        ]
    };
}

macro_rules! gen_xyz {
    () => {
        [
            gen_xyz_for_a!(1),
            gen_xyz_for_a!(2),
            gen_xyz_for_a!(3),
            gen_xyz_for_a!(4),
            gen_xyz_for_a!(5),
            gen_xyz_for_a!(6),
            gen_xyz_for_a!(7),
            gen_xyz_for_a!(8),
            gen_xyz_for_a!(9),
        ]
    };
}

const VAL_XYZ: [[[i32; 10]; 10]; 9] = gen_xyz!();

macro_rules! gen_yz_val {
    ($b:expr, $c:expr) => {
        10 * $b + $c
    };
}

macro_rules! gen_yz_for_b {
    ($b:expr) => {
        [
            gen_yz_val!($b, 0),
            gen_yz_val!($b, 1),
            gen_yz_val!($b, 2),
            gen_yz_val!($b, 3),
            gen_yz_val!($b, 4),
            gen_yz_val!($b, 5),
            gen_yz_val!($b, 6),
            gen_yz_val!($b, 7),
            gen_yz_val!($b, 8),
            gen_yz_val!($b, 9),
        ]
    };
}

macro_rules! gen_yz {
    () => {
        [
            gen_yz_for_b!(1),
            gen_yz_for_b!(2),
            gen_yz_for_b!(3),
            gen_yz_for_b!(4),
            gen_yz_for_b!(5),
            gen_yz_for_b!(6),
            gen_yz_for_b!(7),
            gen_yz_for_b!(8),
            gen_yz_for_b!(9),
        ]
    };
}

const VAL_YZ: [[i32; 10]; 9] = gen_yz!();

macro_rules! gen_xz_val {
    ($a:expr, $c:expr) => {
        10 * $a + $c
    };
}

macro_rules! gen_xz_for_a {
    ($a:expr) => {
        [
            gen_xz_val!($a, 0),
            gen_xz_val!($a, 1),
            gen_xz_val!($a, 2),
            gen_xz_val!($a, 3),
            gen_xz_val!($a, 4),
            gen_xz_val!($a, 5),
            gen_xz_val!($a, 6),
            gen_xz_val!($a, 7),
            gen_xz_val!($a, 8),
            gen_xz_val!($a, 9),
        ]
    };
}

macro_rules! gen_xz {
    () => {
        [
            gen_xz_for_a!(1),
            gen_xz_for_a!(2),
            gen_xz_for_a!(3),
            gen_xz_for_a!(4),
            gen_xz_for_a!(5),
            gen_xz_for_a!(6),
            gen_xz_for_a!(7),
            gen_xz_for_a!(8),
            gen_xz_for_a!(9),
        ]
    };
}

const VAL_XZ: [[i32; 10]; 9] = gen_xz!();

macro_rules! gen_xy_val {
    ($a:expr, $b:expr) => {
        10 * $a + $b
    };
}

macro_rules! gen_xy_for_a {
    ($a:expr) => {
        [
            gen_xy_val!($a, 0),
            gen_xy_val!($a, 1),
            gen_xy_val!($a, 2),
            gen_xy_val!($a, 3),
            gen_xy_val!($a, 4),
            gen_xy_val!($a, 5),
            gen_xy_val!($a, 6),
            gen_xy_val!($a, 7),
            gen_xy_val!($a, 8),
            gen_xy_val!($a, 9),
        ]
    };
}

macro_rules! gen_xy {
    () => {
        [
            gen_xy_for_a!(1),
            gen_xy_for_a!(2),
            gen_xy_for_a!(3),
            gen_xy_for_a!(4),
            gen_xy_for_a!(5),
            gen_xy_for_a!(6),
            gen_xy_for_a!(7),
            gen_xy_for_a!(8),
            gen_xy_for_a!(9),
        ]
    };
}

const VAL_XY: [[i32; 10]; 9] = gen_xy!();

fn main() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();
        let v1 = VAL_XYZ[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }
        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }
        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }
        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
    std::process::exit(0);
}