use std::cell::RefCell;
use std::process;

thread_local! {
    static RNG_STATE: RefCell<u32> = RefCell::new(2463534242);
}

fn rng_next() -> u32 {
    RNG_STATE.with(|state| {
        let mut x = *state.borrow();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *state.borrow_mut() = x;
        x
    })
}

fn pick_1_9() -> i32 {
    (rng_next() % 9) as i32 + 1
}

fn pick_0_9() -> i32 {
    (rng_next() % 10) as i32
}

fn main() {
    let val_xyz: [[[i32; 10]; 10]; 9] = {
        let mut arr = [[[0; 10]; 10]; 9];
        for a in 0..9 {
            for b in 0..10 {
                for c in 0..10 {
                    arr[a][b][c] = 100 * ((a + 1) as i32) + 10 * (b as i32) + c as i32;
                }
            }
        }
        arr
    };

    let val_yz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for b in 0..9 {
            for c in 0..10 {
                arr[b][c] = 10 * ((b + 1) as i32) + c as i32;
            }
        }
        arr
    };

    let val_xz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 0..9 {
            for c in 0..10 {
                arr[a][c] = 10 * ((a + 1) as i32) + c as i32;
            }
        }
        arr
    };

    let val_xy: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 0..9 {
            for b in 0..10 {
                arr[a][b] = 10 * ((a + 1) as i32) + b as i32;
            }
        }
        arr
    };

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            process::exit(4);
        }
    }

    process::exit(0);
}