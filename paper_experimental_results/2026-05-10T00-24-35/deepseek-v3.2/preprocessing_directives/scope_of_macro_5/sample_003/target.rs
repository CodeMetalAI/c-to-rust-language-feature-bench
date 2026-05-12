fn main() {
    const D0: i32 = 0;
    const D1: i32 = 1;
    const D2: i32 = 2;
    const D3: i32 = 3;
    const D4: i32 = 4;
    const D5: i32 = 5;
    const D6: i32 = 6;
    const D7: i32 = 7;
    const D8: i32 = 8;
    const D9: i32 = 9;

    fn pick_d(n: usize) -> i32 {
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
            _ => panic!("Invalid digit"),
        }
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

    let val_xyz: [[[i32; 10]; 10]; 9] = {
        let mut arr = [[[0; 10]; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                for c in 0..=9 {
                    arr[a - 1][b][c] = vxyz(a, b, c);
                }
            }
        }
        arr
    };

    let val_yz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for b in 1..=9 {
            for c in 0..=9 {
                arr[b - 1][c] = vyz(b, c);
            }
        }
        arr
    };

    let val_xz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 1..=9 {
            for c in 0..=9 {
                arr[a - 1][c] = vxz(a, c);
            }
        }
        arr
    };

    let val_xy: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                arr[a - 1][b] = vxy(a, b);
            }
        }
        arr
    };

    let mut rng_state: u32 = 2463534242;
    fn rng_next(state: &mut u32) -> u32 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *state = x;
        x
    }

    fn pick_1_9(state: &mut u32) -> usize {
        (rng_next(state) % 9) as usize + 1
    }

    fn pick_0_9(state: &mut u32) -> usize {
        (rng_next(state) % 10) as usize
    }

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[a - 1][b][c];
        if v1 != (100 * a as i32 + 10 * b as i32 + c as i32) {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[b2 - 1][c2];
        if v2 != (10 * b2 as i32 + c2 as i32) {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[a3 - 1][c3];
        if v3 != (10 * a3 as i32 + c3 as i32) {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[a4 - 1][b4];
        if v4 != (10 * a4 as i32 + b4 as i32) {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}