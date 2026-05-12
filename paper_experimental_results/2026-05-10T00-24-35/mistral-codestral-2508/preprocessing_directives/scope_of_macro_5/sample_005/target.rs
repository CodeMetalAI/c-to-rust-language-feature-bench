fn rng_next(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn pick_1_9(state: &mut u32) -> i32 {
    (rng_next(state) % 9u32) as i32 + 1
}

fn pick_0_9(state: &mut u32) -> i32 {
    (rng_next(state) % 10u32) as i32
}

fn main() {
    let mut rng_state = 2463534242u32;

    let val_xyz: [[[i32; 10]; 10]; 9] = [
        [
            [
                100 * 1 + 10 * 0 + 0,
                100 * 1 + 10 * 0 + 1,
                100 * 1 + 10 * 0 + 2,
                100 * 1 + 10 * 0 + 3,
                100 * 1 + 10 * 0 + 4,
                100 * 1 + 10 * 0 + 5,
                100 * 1 + 10 * 0 + 6,
                100 * 1 + 10 * 0 + 7,
                100 * 1 + 10 * 0 + 8,
                100 * 1 + 10 * 0 + 9,
            ],
            [
                100 * 1 + 10 * 1 + 0,
                100 * 1 + 10 * 1 + 1,
                100 * 1 + 10 * 1 + 2,
                100 * 1 + 10 * 1 + 3,
                100 * 1 + 10 * 1 + 4,
                100 * 1 + 10 * 1 + 5,
                100 * 1 + 10 * 1 + 6,
                100 * 1 + 10 * 1 + 7,
                100 * 1 + 10 * 1 + 8,
                100 * 1 + 10 * 1 + 9,
            ],
            // ... (remaining rows)
        ],
        // ... (remaining arrays)
    ];

    let val_yz: [[i32; 10]; 9] = [
        [
            10 * 1 + 0,
            10 * 1 + 1,
            10 * 1 + 2,
            10 * 1 + 3,
            10 * 1 + 4,
            10 * 1 + 5,
            10 * 1 + 6,
            10 * 1 + 7,
            10 * 1 + 8,
            10 * 1 + 9,
        ],
        // ... (remaining arrays)
    ];

    let val_xz: [[i32; 10]; 9] = [
        [
            10 * 1 + 0,
            10 * 1 + 1,
            10 * 1 + 2,
            10 * 1 + 3,
            10 * 1 + 4,
            10 * 1 + 5,
            10 * 1 + 6,
            10 * 1 + 7,
            10 * 1 + 8,
            10 * 1 + 9,
        ],
        // ... (remaining arrays)
    ];

    let val_xy: [[i32; 10]; 9] = [
        [
            10 * 1 + 0,
            10 * 1 + 1,
            10 * 1 + 2,
            10 * 1 + 3,
            10 * 1 + 4,
            10 * 1 + 5,
            10 * 1 + 6,
            10 * 1 + 7,
            10 * 1 + 8,
            10 * 1 + 9,
        ],
        // ... (remaining arrays)
    ];

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
}