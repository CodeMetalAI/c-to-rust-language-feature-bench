use std::cell::RefCell;

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

fn init_val_xyz() -> [[[i32; 10]; 10]; 9] {
    let mut arr = [[[0; 10]; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                arr[(a - 1) as usize][b as usize][c as usize] = 100 * a + 10 * b + c;
            }
        }
    }
    arr
}

fn init_val_yz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for b in 1..=9 {
        for c in 0..=9 {
            arr[(b - 1) as usize][c as usize] = 10 * b + c;
        }
    }
    arr
}

fn init_val_xz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for a in 1..=9 {
        for c in 0..=9 {
            arr[(a - 1) as usize][c as usize] = 10 * a + c;
        }
    }
    arr
}

fn init_val_xy() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            arr[(a - 1) as usize][b as usize] = 10 * a + b;
        }
    }
    arr
}

fn main() {
    let val_xyz = init_val_xyz();
    let val_yz = init_val_yz();
    let val_xz = init_val_xz();
    let val_xy = init_val_xy();

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}