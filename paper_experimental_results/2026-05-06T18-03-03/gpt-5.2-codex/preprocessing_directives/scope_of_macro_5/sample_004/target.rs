use std::process::exit;

fn rng_next(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
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
    let mut rng_state: u32 = 2463534242u32;

    let mut val_xyz = vec![vec![vec![0i32; 10]; 10]; 9];
    let mut val_yz = vec![vec![0i32; 10]; 9];
    let mut val_xz = vec![vec![0i32; 10]; 9];
    let mut val_xy = vec![vec![0i32; 10]; 9];

    for a in 1..=9 {
        let ai = (a - 1) as usize;
        for b in 0..=9 {
            let bi = b as usize;
            for c in 0..=9 {
                let ci = c as usize;
                val_xyz[ai][bi][ci] = 100 * a + 10 * b + c;
            }
            val_xy[ai][bi] = 10 * a + b;
        }
        for c in 0..=9 {
            let ci = c as usize;
            val_xz[ai][ci] = 10 * a + c;
        }
    }

    for b in 1..=9 {
        let bi = (b - 1) as usize;
        for c in 0..=9 {
            let ci = c as usize;
            val_yz[bi][ci] = 10 * b + c;
        }
    }

    let mut exit_code = 0;
    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            exit_code = 1;
            break;
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            exit_code = 2;
            break;
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            exit_code = 3;
            break;
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            exit_code = 4;
            break;
        }
    }

    if exit_code != 0 {
        exit(exit_code);
    }
}