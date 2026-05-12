fn rng_next(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn pick_1_9(state: &mut u32) -> u8 {
    (rng_next(state) as u8 % 9) + 1
}

fn pick_0_9(state: &mut u32) -> u8 {
    rng_next(state) as u8 % 10
}

fn main() {
    let mut rng_state = 2463534242u;

    let val_xyz: [[[u16; 10]; 10]; 9] = {
        let mut arr = [[[0; 10]; 10]; 9];
        for i in 0..9 {
            for j in 0..10 {
                for k in 0..10 {
                    arr[i][j][k] = 100 * (i + 1) as u16 + 10 * j as u16 + k as u16;
                }
            }
        }
        arr
    };

    let val_yz: [[u16; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for i in 0..9 {
            for j in 0..10 {
                arr[i][j] = 10 * (i + 1) as u16 + j as u16;
            }
        }
        arr
    };

    let val_xz: [[u16; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for i in 0..9 {
            for j in 0..10 {
                arr[i][j] = 10 * (i + 1) as u16 + j as u16;
            }
        }
        arr
    };

    let val_xy: [[u16; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for i in 0..9 {
            for j in 0..10 {
                arr[i][j] = 10 * (i + 1) as u16 + j as u16;
            }
        }
        arr
    };

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1!= (100 * a as u16 + 10 * b as u16 + c as u16) {
            return;
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2!= (10 * b2 as u16 + c2 as u16) {
            return;
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3!= (10 * a3 as u16 + c3 as u16) {
            return;
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4!= (10 * a4 as u16 + b4 as u16) {
            return;
        }
    }
}