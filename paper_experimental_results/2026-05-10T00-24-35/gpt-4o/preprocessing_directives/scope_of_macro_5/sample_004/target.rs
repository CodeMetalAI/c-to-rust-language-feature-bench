fn pick_d(n: usize) -> i32 {
    n as i32
}

fn vxyz(a: usize, b: usize, c: usize) -> i32 {
    100 * pick_d(a) + 10 * pick_d(b) + pick_d(c)
}

fn vyz(b: usize, c: usize) -> i32 {
    10 * pick_d(b) + pick_d(c)
}

fn vxz(a: usize, c: usize) -> i32 {
    10 * pick_d(a) + pick_d(c)
}

fn vxy(a: usize, b: usize) -> i32 {
    10 * pick_d(a) + pick_d(b)
}

fn rng_next(rng_state: &mut u32) -> u32 {
    let mut x = *rng_state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *rng_state = x;
    x
}

fn pick_1_9(rng_state: &mut u32) -> i32 {
    (rng_next(rng_state) % 9 + 1) as i32
}

fn pick_0_9(rng_state: &mut u32) -> i32 {
    (rng_next(rng_state) % 10) as i32
}

fn main() {
    let mut rng_state = 2463534242u32;

    let val_xyz: [[[i32; 10]; 10]; 9] = (1..=9)
        .map(|a| {
            (0..=9)
                .map(|b| {
                    (0..=9).map(|c| vxyz(a, b, c)).collect::<Vec<i32>>().try_into().unwrap()
                })
                .collect::<Vec<[i32; 10]>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[[i32; 10]; 10]>>()
        .try_into()
        .unwrap();

    let val_yz: [[i32; 10]; 9] = (1..=9)
        .map(|b| {
            (0..=9).map(|c| vyz(b, c)).collect::<Vec<i32>>().try_into().unwrap()
        })
        .collect::<Vec<[i32; 10]>>()
        .try_into()
        .unwrap();

    let val_xz: [[i32; 10]; 9] = (1..=9)
        .map(|a| {
            (0..=9).map(|c| vxz(a, c)).collect::<Vec<i32>>().try_into().unwrap()
        })
        .collect::<Vec<[i32; 10]>>()
        .try_into()
        .unwrap();

    let val_xy: [[i32; 10]; 9] = (1..=9)
        .map(|a| {
            (0..=9).map(|b| vxy(a, b)).collect::<Vec<i32>>().try_into().unwrap()
        })
        .collect::<Vec<[i32; 10]>>()
        .try_into()
        .unwrap();

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state) as usize;
        let b = pick_0_9(&mut rng_state) as usize;
        let c = pick_0_9(&mut rng_state) as usize;

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a as i32 + 10 * b as i32 + c as i32 {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state) as usize;
        let c2 = pick_0_9(&mut rng_state) as usize;
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 as i32 + c2 as i32 {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state) as usize;
        let c3 = pick_0_9(&mut rng_state) as usize;
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 as i32 + c3 as i32 {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state) as usize;
        let b4 = pick_0_9(&mut rng_state) as usize;
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 as i32 + b4 as i32 {
            std::process::exit(4);
        }
    }
}