use std::process::exit;

fn main() {
    let mut rng_state: u32 = 2463534242u32;

    let mut rng_next = || -> u32 {
        let mut x = rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng_state = x;
        x
    };

    let mut pick_1_9 = || -> i32 { (rng_next() % 9) as i32 + 1 };
    let mut pick_0_9 = || -> i32 { (rng_next() % 10) as i32 };

    let mut val_xyz = [[[0i32; 10]; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                val_xyz[(a - 1) as usize][b as usize][c as usize] =
                    100 * a + 10 * b + c;
            }
        }
    }

    let mut val_yz = [[0i32; 10]; 9];
    for b in 1..=9 {
        for c in 0..=9 {
            val_yz[(b - 1) as usize][c as usize] = 10 * b + c;
        }
    }

    let mut val_xz = [[0i32; 10]; 9];
    for a in 1..=9 {
        for c in 0..=9 {
            val_xz[(a - 1) as usize][c as usize] = 10 * a + c;
        }
    }

    let mut val_xy = [[0i32; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            val_xy[(a - 1) as usize][b as usize] = 10 * a + b;
        }
    }

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            exit(4);
        }
    }
}