fn main() {
    let mut rng_state = 2463534242u32;

    let mut rng_next = || {
        let mut x = rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng_state = x;
        x
    };

    let pick_1_9 = || (rng_next() % 9 + 1) as usize;
    let pick_0_9 = || (rng_next() % 10) as usize;

    let val_xyz: Vec<Vec<Vec<i32>>> = (1..=9).map(|a| {
        (0..10).map(|b| {
            (0..10).map(|c| 100 * a + 10 * b + c as i32).collect()
        }).collect()
    }).collect();

    let val_yz: Vec<Vec<i32>> = (1..=9).map(|b| {
        (0..10).map(|c| 10 * b + c as i32).collect()
    }).collect();

    let val_xz: Vec<Vec<i32>> = (1..=9).map(|a| {
        (0..10).map(|c| 10 * a + c as i32).collect()
    }).collect();

    let val_xy: Vec<Vec<i32>> = (1..=9).map(|a| {
        (0..10).map(|b| 10 * a + b as i32).collect()
    }).collect();

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a as i32 + 10 * b as i32 + c as i32 {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 as i32 + c2 as i32 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 as i32 + c3 as i32 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 as i32 + b4 as i32 {
            std::process::exit(4);
        }
    }
}