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

fn v_xyz(a: usize, b: usize, c: usize) -> usize {
    let d_a = pick_d(a);
    let d_b = pick_d(b);
    let d_c = pick_d(c);
    d_a * 100 + d_b * 10 + d_c
}

fn v_yz(b: usize, c: usize) -> usize {
    let d_b = pick_d(b);
    let d_c = pick_d(c);
    d_b * 10 + d_c
}

fn v_xz(a: usize, c: usize) -> usize {
    let d_a = pick_d(a);
    let d_c = pick_d(c);
    d_a * 10 + d_c
}

fn v_xy(a: usize, b: usize) -> usize {
    let d_a = pick_d(a);
    let d_b = pick_d(b);
    d_a * 10 + d_b
}

static mut RNG_STATE: u32 = 2463534242;

fn rng_next() -> u32 {
    unsafe {
        let mut x = RNG_STATE;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        RNG_STATE = x;
        x
    }
}

fn pick_1_9() -> usize {
    (rng_next() % 9) as usize + 1
}

fn pick_0_9() -> usize {
    (rng_next() % 10) as usize
}

fn main() -> i32 {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = v_xyz(a - 1, b, c);
        if v1 != 100 * a + 10 * b + c {
            return 1;
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = v_yz(b2 - 1, c2);
        if v2 != 10 * b2 + c2 {
            return 2;
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = v_xz(a3 - 1, c3);
        if v3 != 10 * a3 + c3 {
            return 3;
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = v_xy(a4 - 1, b4);
        if v4 != 10 * a4 + b4 {
            return 4;
        }
    }

    0
}