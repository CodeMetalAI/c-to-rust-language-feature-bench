fn main() {
    // Initialization of arrays with calculated numbers
    let val_xyz = init_val_xyz();
    let val_yz = init_val_yz();
    let val_xz = init_val_xz();
    let val_xy = init_val_xy();

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
}

fn init_val_xyz() -> [[[i32; 10]; 10]; 9] {
    let mut arr = [[[0; 10]; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                arr[a - 1][b][c] = 100 * a + 10 * b + c;
            }
        }
    }
    arr
}

fn init_val_yz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for b in 1..=9 {
        for c in 0..=9 {
            arr[b - 1][c] = 10 * b + c;
        }
    }
    arr
}

fn init_val_xz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for a in 1..=9 {
        for c in 0..=9 {
            arr[a - 1][c] = 10 * a + c;
        }
    }
    arr
}

fn init_val_xy() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            arr[a - 1][b] = 10 * a + b;
        }
    }
    arr
}

fn pick_1_9() -> i32 {
    rng_next() as i32 % 9 + 1
}

fn pick_0_9() -> i32 {
    rng_next() as i32 % 10
}

static mut RNG_STATE: u32 = 2463534242;

fn rng_next() -> u32 {
    unsafe {
        let mut x = RNG_STATE;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        RNG_STATE = x;
        x
    }
}