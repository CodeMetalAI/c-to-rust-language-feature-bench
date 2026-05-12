use std::process::exit;

fn rng_next(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x.wrapping_shl(13);
    x ^= x.wrapping_shr(17);
    x ^= x.wrapping_shl(5);
    *state = x;
    x
}

fn pick_1_9(state: &mut u32) -> i32 {
    (rng_next(state) % 9 + 1) as i32
}

fn pick_0_9(state: &mut u32) -> i32 {
    (rng_next(state) % 10) as i32
}

fn main() {
    let mut val_xyz = [[[0i32; 10]; 10]; 9];
    for a in 0..9 {
        for b in 0..10 {
            for c in 0..10 {
                val_xyz[a][b][c] = 100 * (a as i32 + 1) + 10 * b as i32 + c as i32;
            }
        }
    }

    let mut val_yz = [[0i32; 10]; 9];
    for b in 0..9 {
        for c in 0..10 {
            val_yz[b][c] = 10 * (b as i32 + 1) + c as i32;
        }
    }

    let mut val_xz = [[0i32; 10]; 9];
    for a in 0..9 {
        for c in 0..10 {
            val_xz[a][c] = 10 * (a as i32 + 1) + c as i32;
        }
    }

    let mut val_xy = [[0i32; 10]; 9];
    for a in 0..9 {
        for b in 0..10 {
            val_xy[a][b] = 10 * (a as i32 + 1) + b as i32;
        }
    }

    let mut rng_state: u32 = 2463534242u32;

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            exit(1);
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            exit(4);
        }
    }

    exit(0);
}