use std::array;

fn rng_next(state: &mut u32) -> u32 {
    let x = *state;
    let x = x ^ (x << 13);
    let x = x ^ (x >> 17);
    let x = x ^ (x << 5);
    *state = x;
    x
}

fn pick_1_9(state: &mut u32) -> i32 {
    ((rng_next(state) % 9) as i32) + 1
}

fn pick_0_9(state: &mut u32) -> i32 {
    (rng_next(state) % 10) as i32
}

fn main() {
    let mut rng_state: u32 = 2463534242;

    let val_xyz: [[[i32; 10]; 10]; 9] = array::from_fn(|i| {
        array::from_fn(|j| {
            array::from_fn(|k| 100 * ((i as i32) + 1) + 10 * (j as i32) + (k as i32))
        })
    });

    let val_yz: [[i32; 10]; 9] = array::from_fn(|i| {
        array::from_fn(|j| 10 * ((i as i32) + 1) + (j as i32))
    });

    let val_xz: [[i32; 10]; 9] = array::from_fn(|i| {
        array::from_fn(|j| 10 * ((i as i32) + 1) + (j as i32))
    });

    let val_xy: [[i32; 10]; 9] = array::from_fn(|i| {
        array::from_fn(|j| 10 * ((i as i32) + 1) + (j as i32))
    });

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