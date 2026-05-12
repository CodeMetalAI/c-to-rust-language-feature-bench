fn main() {
    let mut rng_state: u32 = 2463534242;
    
    let mut rng_next = || -> u32 {
        let mut x = rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng_state = x;
        x
    };
    
    let mut pick_1_9 = || -> i32 {
        (rng_next() % 9) as i32 + 1
    };
    
    let mut pick_0_9 = || -> i32 {
        (rng_next() % 10) as i32
    };
    
    // val_xyz[a-1][b][c] = 100*a + 10*b + c where a in 1..=9, b in 0..=9, c in 0..=9
    let val_xyz: [[[i32; 10]; 10]; 9] = {
        let mut arr = [[[0i32; 10]; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                for c in 0..=9 {
                    arr[a - 1][b][c] = 100 * (a as i32) + 10 * (b as i32) + (c as i32);
                }
            }
        }
        arr
    };
    
    // val_yz[b-1][c] = 10*b + c where b in 1..=9, c in 0..=9
    let val_yz: [[i32; 10]; 9] = {
        let mut arr = [[0i32; 10]; 9];
        for b in 1..=9 {
            for c in 0..=9 {
                arr[b - 1][c] = 10 * (b as i32) + (c as i32);
            }
        }
        arr
    };
    
    // val_xz[a-1][c] = 10*a + c where a in 1..=9, c in 0..=9
    let val_xz: [[i32; 10]; 9] = {
        let mut arr = [[0i32; 10]; 9];
        for a in 1..=9 {
            for c in 0..=9 {
                arr[a - 1][c] = 10 * (a as i32) + (c as i32);
            }
        }
        arr
    };
    
    // val_xy[a-1][b] = 10*a + b where a in 1..=9, b in 0..=9
    let val_xy: [[i32; 10]; 9] = {
        let mut arr = [[0i32; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                arr[a - 1][b] = 10 * (a as i32) + (b as i32);
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