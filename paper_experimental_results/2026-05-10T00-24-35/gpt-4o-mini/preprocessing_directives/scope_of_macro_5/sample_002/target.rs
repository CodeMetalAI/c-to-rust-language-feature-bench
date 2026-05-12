const D0: usize = 0;
const D1: usize = 1;
const D2: usize = 2;
const D3: usize = 3;
const D4: usize = 4;
const D5: usize = 5;
const D6: usize = 6;
const D7: usize = 7;
const D8: usize = 8;
const D9: usize = 9;

fn pick_d(n: usize) -> usize {
    match n {
        0 => D0,
        1 => D1,
        2 => D2,
        3 => D3,
        4 => D4,
        5 => D5,
        6 => D6,
        7 => D7,
        8 => D8,
        9 => D9,
        _ => unreachable!(),
    }
}

fn vxyz(a: usize, b: usize, c: usize) -> usize {
    let d_a = pick_d(a);
    let d_b = pick_d(b);
    let d_c = pick_d(c);
    d_a * 100 + d_b * 10 + d_c
}

fn vyz(b: usize, c: usize) -> usize {
    let d_b = pick_d(b);
    let d_c = pick_d(c);
    d_b * 10 + d_c
}

fn vxz(a: usize, c: usize) -> usize {
    let d_a = pick_d(a);
    let d_c = pick_d(c);
    d_a * 10 + d_c
}

fn vxy(a: usize, b: usize) -> usize {
    let d_a = pick_d(a);
    let d_b = pick_d(b);
    d_a * 10 + d_b
}

static mut RNG_STATE: u32 = 2463534242;

fn rng_next() -> u32 {
    unsafe {
        let x = RNG_STATE;
        let mut x = x ^ (x << 13);
        x ^= x >> 17;
        x ^= x << 5;
        RNG_STATE = x;
        x
    }
}

fn pick_1_9() -> usize {
    (rng_next() as usize % 9) + 1
}

fn pick_0_9() -> usize {
    (rng_next() as usize % 10)
}

const VAL_XYZ: [[[usize; 10]; 10]; 9] = [
    [vxyz(1, 0, 0), vxyz(1, 0, 1), vxyz(1, 0, 2), vxyz(1, 0, 3), vxyz(1, 0, 4), vxyz(1, 0, 5), vxyz(1, 0, 6), vxyz(1, 0, 7), vxyz(1, 0, 8), vxyz(1, 0, 9)],
    [vxyz(1, 1, 0), vxyz(1, 1, 1), vxyz(1, 1, 2), vxyz(1, 1, 3), vxyz(1, 1, 4), vxyz(1, 1, 5), vxyz(1, 1, 6), vxyz(1, 1, 7), vxyz(1, 1, 8), vxyz(1, 1, 9)],
    [vxyz(1, 2, 0), vxyz(1, 2, 1), vxyz(1, 2, 2), vxyz(1, 2, 3), vxyz(1, 2, 4), vxyz(1, 2, 5), vxyz(1, 2, 6), vxyz(1, 2, 7), vxyz(1, 2, 8), vxyz(1, 2, 9)],
    [vxyz(1, 3, 0), vxyz(1, 3, 1), vxyz(1, 3, 2), vxyz(1, 3, 3), vxyz(1, 3, 4), vxyz(1, 3, 5), vxyz(1, 3, 6), vxyz(1, 3, 7), vxyz(1, 3, 8), vxyz(1, 3, 9)],
    [vxyz(1, 4, 0), vxyz(1, 4, 1), vxyz(1, 4, 2), vxyz(1, 4, 3), vxyz(1, 4, 4), vxyz(1, 4, 5), vxyz(1, 4, 6), vxyz(1, 4, 7), vxyz(1, 4, 8), vxyz(1, 4, 9)],
    [vxyz(1, 5, 0), vxyz(1, 5, 1), vxyz(1, 5, 2), vxyz(1, 5, 3), vxyz(1, 5, 4), vxyz(1, 5, 5), vxyz(1, 5, 6), vxyz(1, 5, 7), vxyz(1, 5, 8), vxyz(1, 5, 9)],
    [vxyz(1, 6, 0), vxyz(1, 6, 1), vxyz(1, 6, 2), vxyz(1, 6, 3), vxyz(1, 6, 4), vxyz(1, 6, 5), vxyz(1, 6, 6), vxyz(1, 6, 7), vxyz(1, 6, 8), vxyz(1, 6, 9)],
    [vxyz(1, 7, 0), vxyz(1, 7, 1), vxyz(1, 7, 2), vxyz(1, 7, 3), vxyz(1, 7, 4), vxyz(1, 7, 5), vxyz(1, 7, 6), vxyz(1, 7, 7), vxyz(1, 7, 8), vxyz(1, 7, 9)],
    [vxyz(1, 8, 0), vxyz(1, 8, 1), vxyz(1, 8, 2), vxyz(1, 8, 3), vxyz(1, 8, 4), vxyz(1, 8, 5), vxyz(1, 8, 6), vxyz(1, 8, 7), vxyz(1, 8, 8), vxyz(1, 8, 9)],
    [vxyz(1, 9, 0), vxyz(1, 9, 1), vxyz(1, 9, 2), vxyz(1, 9, 3), vxyz(1, 9, 4), vxyz(1, 9, 5), vxyz(1, 9, 6), vxyz(1, 9, 7), vxyz(1, 9, 8), vxyz(1, 9, 9)]
];

const VAL_YZ: [[usize; 10]; 9] = [
    [vyz(1, 0), vyz(1, 1), vyz(1, 2), vyz(1, 3), vyz(1, 4), vyz(1, 5), vyz(1, 6), vyz(1, 7), vyz(1, 8), vyz(1, 9)],
    [vyz(2, 0), vyz(2, 1), vyz(2, 2), vyz(2, 3), vyz(2, 4), vyz(2, 5), vyz(2, 6), vyz(2, 7), vyz(2, 8), vyz(2, 9)],
    [vyz(3, 0), vyz(3, 1), vyz(3, 2), vyz(3, 3), vyz(3, 4), vyz(3, 5), vyz(3, 6), vyz(3, 7), vyz(3, 8), vyz(3, 9)],
    [vyz(4, 0), vyz(4, 1), vyz(4, 2), vyz(4, 3), vyz(4, 4), vyz(4, 5), vyz(4, 6), vyz(4, 7), vyz(4, 8), vyz(4, 9)],
    [vyz(5, 0), vyz(5, 1), vyz(5, 2), vyz(5, 3), vyz(5, 4), vyz(5, 5), vyz(5, 6), vyz(5, 7), vyz(5, 8), vyz(5, 9)],
    [vyz(6, 0), vyz(6, 1), vyz(6, 2), vyz(6, 3), vyz(6, 4), vyz(6, 5), vyz(6, 6), vyz(6, 7), vyz(6, 8), vyz(6, 9)],
    [vyz(7, 0), vyz(7, 1), vyz(7, 2), vyz(7, 3), vyz(7, 4), vyz(7, 5), vyz(7, 6), vyz(7, 7), vyz(7, 8), vyz(7, 9)],
    [vyz(8, 0), vyz(8, 1), vyz(8, 2), vyz(8, 3), vyz(8, 4), vyz(8, 5), vyz(8, 6), vyz(8, 7), vyz(8, 8), vyz(8, 9)],
    [vyz(9, 0), vyz(9, 1), vyz(9, 2), vyz(9, 3), vyz(9, 4), vyz(9, 5), vyz(9, 6), vyz(9, 7), vyz(9, 8), vyz(9, 9)]
];

const VAL_XZ: [[usize; 10]; 9] = [
    [vxz(1, 0), vxz(1, 1), vxz(1, 2), vxz(1, 3), vxz(1, 4), vxz(1, 5), vxz(1, 6), vxz(1, 7), vxz(1, 8), vxz(1, 9)],
    [vxz(2, 0), vxz(2, 1), vxz(2, 2), vxz(2, 3), vxz(2, 4), vxz(2, 5), vxz(2, 6), vxz(2, 7), vxz(2, 8), vxz(2, 9)],
    [vxz(3, 0), vxz(3, 1), vxz(3, 2), vxz(3, 3), vxz(3, 4), vxz(3, 5), vxz(3, 6), vxz(3, 7), vxz(3, 8), vxz(3, 9)],
    [vxz(4, 0), vxz(4, 1), vxz(4, 2), vxz(4, 3), vxz(4, 4), vxz(4, 5), vxz(4, 6), vxz(4, 7), vxz(4, 8), vxz(4, 9)],
    [vxz(5, 0), vxz(5, 1), vxz(5, 2), vxz(5, 3), vxz(5, 4), vxz(5, 5), vxz(5, 6), vxz(5, 7), vxz(5, 8), vxz(5, 9)],
    [vxz(6, 0), vxz(6, 1), vxz(6, 2), vxz(6, 3), vxz(6, 4), vxz(6, 5), vxz(6, 6), vxz(6, 7), vxz(6, 8), vxz(6, 9)],
    [vxz(7, 0), vxz(7, 1), vxz(7, 2), vxz(7, 3), vxz(7, 4), vxz(7, 5), vxz(7, 6), vxz(7, 7), vxz(7, 8), vxz(7, 9)],
    [vxz(8, 0), vxz(8, 1), vxz(8, 2), vxz(8, 3), vxz(8, 4), vxz(8, 5), vxz(8, 6), vxz(8, 7), vxz(8, 8), vxz(8, 9)],
    [vxz(9, 0), vxz(9, 1), vxz(9, 2), vxz(9, 3), vxz(9, 4), vxz(9, 5), vxz(9, 6), vxz(9, 7), vxz(9, 8), vxz(9, 9)]
];

const VAL_XY: [[usize; 10]; 9] = [
    [vxy(1, 0), vxy(1, 1), vxy(1, 2), vxy(1, 3), vxy(1, 4), vxy(1, 5), vxy(1, 6), vxy(1, 7), vxy(1, 8), vxy(1, 9)],
    [vxy(2, 0), vxy(2, 1), vxy(2, 2), vxy(2, 3), vxy(2, 4), vxy(2, 5), vxy(2, 6), vxy(2, 7), vxy(2, 8), vxy(2, 9)],
    [vxy(3, 0), vxy(3, 1), vxy(3, 2), vxy(3, 3), vxy(3, 4), vxy(3, 5), vxy(3, 6), vxy(3, 7), vxy(3, 8), vxy(3, 9)],
    [vxy(4, 0), vxy(4, 1), vxy(4, 2), vxy(4, 3), vxy(4, 4), vxy(4, 5), vxy(4, 6), vxy(4, 7), vxy(4, 8), vxy(4, 9)],
    [vxy(5, 0), vxy(5, 1), vxy(5, 2), vxy(5, 3), vxy(5, 4), vxy(5, 5), vxy(5, 6), vxy(5, 7), vxy(5, 8), vxy(5, 9)],
    [vxy(6, 0), vxy(6, 1), vxy(6, 2), vxy(6, 3), vxy(6, 4), vxy(6, 5), vxy(6, 6), vxy(6, 7), vxy(6, 8), vxy(6, 9)],
    [vxy(7, 0), vxy(7, 1), vxy(7, 2), vxy(7, 3), vxy(7, 4), vxy(7, 5), vxy(7, 6), vxy(7, 7), vxy(7, 8), vxy(7, 9)],
    [vxy(8, 0), vxy(8, 1), vxy(8, 2), vxy(8, 3), vxy(8, 4), vxy(8, 5), vxy(8, 6), vxy(8, 7), vxy(8, 8), vxy(8, 9)],
    [vxy(9, 0), vxy(9, 1), vxy(9, 2), vxy(9, 3), vxy(9, 4), vxy(9, 5), vxy(9, 6), vxy(9, 7), vxy(9, 8), vxy(9, 9)]
];

fn main() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = VAL_XYZ[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}