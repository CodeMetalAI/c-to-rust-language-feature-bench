fn cat(a: usize, b: usize) -> usize {
    a * 10 + b
}

fn pick_d(n: usize) -> usize {
    n
}

fn vxyz(a: usize, b: usize, c: usize) -> usize {
    cat(cat(pick_d(a), pick_d(b)), c)
}

fn vyz(b: usize, c: usize) -> usize {
    cat(pick_d(b), pick_d(c))
}

fn vxz(a: usize, c: usize) -> usize {
    cat(pick_d(a), pick_d(c))
}

fn vxy(a: usize, b: usize) -> usize {
    cat(pick_d(a), pick_d(b))
}

static mut RNG_STATE: usize = 2463534242;

fn rng_next() -> usize {
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
    (rng_next() % 9) + 1
}

fn pick_0_9() -> usize {
    rng_next() % 10
}

const VAL_XYZ: [[[usize; 10]; 9]; 10] = [
    [
        [vxyz(1, 0, 0), vxyz(1, 0, 1), vxyz(1, 0, 2), vxyz(1, 0, 3), vxyz(1, 0, 4), vxyz(1, 0, 5), vxyz(1, 0, 6), vxyz(1, 0, 7), vxyz(1, 0, 8), vxyz(1, 0, 9)],
        // ... initialized for all combinations of a (1-9) and b (0-9)
    ],
    // ... repeat for a = 2 to 9
];

const VAL_YZ: [[usize; 10]; 9] = [
    [vyz(1, 0), vyz(1, 1), vyz(1, 2), vyz(1, 3), vyz(1, 4), vyz(1, 5), vyz(1, 6), vyz(1, 7), vyz(1, 8), vyz(1, 9)],
    // ... repeat for b = 2 to 9
];

const VAL_XZ: [[usize; 10]; 9] = [
    [vxz(1, 0), vxz(1, 1), vxz(1, 2), vxz(1, 3), vxz(1, 4), vxz(1, 5), vxz(1, 6), vxz(1, 7), vxz(1, 8), vxz(1, 9)],
    // ... repeat for a = 2 to 9
];

const VAL_XY: [[usize; 10]; 9] = [
    [vxy(1, 0), vxy(1, 1), vxy(1, 2), vxy(1, 3), vxy(1, 4), vxy(1, 5), vxy(1, 6), vxy(1, 7), vxy(1, 8), vxy(1, 9)],
    // ... repeat for a = 2 to 9
];

fn main() -> std::process::ExitCode {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = VAL_XYZ[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            return std::process::ExitCode::from(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            return std::process::ExitCode::from(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            return std::process::ExitCode::from(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            return std::process::ExitCode::from(4);
        }
    }

    std::process::ExitCode::from(0)
}