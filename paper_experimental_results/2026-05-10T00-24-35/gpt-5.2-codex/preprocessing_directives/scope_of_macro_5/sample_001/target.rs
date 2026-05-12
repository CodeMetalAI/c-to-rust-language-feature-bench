use std::process::exit;

struct Rng {
    state: u32,
}

impl Rng {
    fn new() -> Self {
        Self { state: 2463534242u32 }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn pick_1_9(&mut self) -> i32 {
        (self.next() % 9 + 1) as i32
    }

    fn pick_0_9(&mut self) -> i32 {
        (self.next() % 10) as i32
    }
}

fn main() {
    let mut val_xyz = [[[0i32; 10]; 10]; 9];
    for a in 0..9 {
        for b in 0..10 {
            for c in 0..10 {
                let aa = (a + 1) as i32;
                val_xyz[a][b][c] = 100 * aa + 10 * b as i32 + c as i32;
            }
        }
    }

    let mut val_yz = [[0i32; 10]; 9];
    for b in 0..9 {
        for c in 0..10 {
            let bb = (b + 1) as i32;
            val_yz[b][c] = 10 * bb + c as i32;
        }
    }

    let mut val_xz = [[0i32; 10]; 9];
    for a in 0..9 {
        for c in 0..10 {
            let aa = (a + 1) as i32;
            val_xz[a][c] = 10 * aa + c as i32;
        }
    }

    let mut val_xy = [[0i32; 10]; 9];
    for a in 0..9 {
        for b in 0..10 {
            let aa = (a + 1) as i32;
            val_xy[a][b] = 10 * aa + b as i32;
        }
    }

    let mut rng = Rng::new();

    for _ in 0..50 {
        let a = rng.pick_1_9();
        let b = rng.pick_0_9();
        let c = rng.pick_0_9();

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            exit(1);
        }

        let b2 = rng.pick_1_9();
        let c2 = rng.pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            exit(2);
        }

        let a3 = rng.pick_1_9();
        let c3 = rng.pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            exit(3);
        }

        let a4 = rng.pick_1_9();
        let b4 = rng.pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            exit(4);
        }
    }

    exit(0);
}