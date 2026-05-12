fn main() {
    const D: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    fn vxyz(a: usize, b: usize, c: usize) -> usize {
        D[a] * 100 + D[b] * 10 + D[c]
    }
    
    fn vyz(b: usize, c: usize) -> usize {
        D[b] * 10 + D[c]
    }
    
    fn vxz(a: usize, c: usize) -> usize {
        D[a] * 10 + D[c]
    }
    
    fn vxy(a: usize, b: usize) -> usize {
        D[a] * 10 + D[b]
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
    
    fn pick_1_9() -> usize {
        (rng_next() % 9 + 1) as usize
    }
    
    fn pick_0_9() -> usize {
        (rng_next() % 10) as usize
    }
    
    let mut val_xyz = [[[0; 10]; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                val_xyz[a - 1][b][c] = vxyz(a, b, c);
            }
        }
    }
    
    let mut val_yz = [[0; 10]; 9];
    for b in 1..=9 {
        for c in 0..=9 {
            val_yz[b - 1][c] = vyz(b, c);
        }
    }
    
    let mut val_xz = [[0; 10]; 9];
    for a in 1..=9 {
        for c in 0..=9 {
            val_xz[a - 1][c] = vxz(a, c);
        }
    }
    
    let mut val_xy = [[0; 10]; 9];
    for a in 1..=9 {
        for b in 0..=9 {
            val_xy[a - 1][b] = vxy(a, b);
        }
    }

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