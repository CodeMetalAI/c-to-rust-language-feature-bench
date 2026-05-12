use std::process;

fn rng_next(state: &mut u32) -> u32 {
    let x = *state;
    *state = x ^ (x << 13) ^ (x >> 17) ^ (x << 5);
    *state
}

fn pick_1_9(state: &mut u32) -> i32 {
    (rng_next(state) % 9) as i32 + 1
}

fn pick_0_9(state: &mut u32) -> i32 {
    (rng_next(state) % 10) as i32
}

fn main() {
    let mut rng_state = 2463534242u32;

    let val_xyz = {
        let mut arr = [[[0i32; 10]; 10]; 9];
        for a in 0..9 {
            for b in 0..10 {
                for c in 0..10 {
                    arr[a][b][c] = 100 * (a as i32 + 1) + 10 * (b as i32) + c as i32;
                }
            }
        }
        arr
    };

    let val_yz = {
        let mut arr = [[0i32; 10]; 9];
        for b in 0..9 {
            for c in 0..10 {
                arr[b][c] = 10 * (b as i32 + 1) + c as i32;
            }
        }
        arr
    };

    let val_xz = {
        let mut arr = [[0i32; 10]; 9];
        for a in 0..9 {
            for c in 0..10 {
                arr[a][c] = 10 * (a as i32 + 1) + c as i32;
            }
        }
        arr
    };

    let val_xy = {
        let mut arr = [[0i32; 10]; 9];
        for a in 0..9 {
            for b in 0..10 {
                arr[a][b] = 10 * (a as i32 + 1) + b as i32;
            }
        }
        arr
    };

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            process::exit(4);
        }
    }

    process::exit(0);
}