struct Rng {
    state: u32,
}

impl Rng {
    fn new() -> Self {
        Self { state: 2463534242 }
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
        (self.next() % 9) as i32 + 1
    }

    fn pick_0_9(&mut self) -> i32 {
        (self.next() % 10) as i32
    }
}

fn val_xyz() -> [[[i32; 10]; 10]; 9] {
    let mut arr = [[[0i32; 10]; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            for k in 0..10 {
                arr[i][j][k] = 100 * (i as i32 + 1) + 10 * j as i32 + k as i32;
            }
        }
    }
    arr
}

fn val_yz() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i as i32 + 1) + j as i32;
        }
    }
    arr
}

fn val_xz() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i as i32 + 1) + j as i32;
        }
    }
    arr
}

fn val_xy() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i as i32 + 1) + j as i32;
        }
    }
    arr
}

fn main() {
    let val_xyz = val_xyz();
    let val_yz = val_yz();
    let val_xz = val_xz();
    let val_xy = val_xy();
    let mut rng = Rng::new();
    for _ in 0..50 {
        let a = rng.pick_1_9();
        let b = rng.pick_0_9();
        let c = rng.pick_0_9();
        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }
        let b2 = rng.pick_1_9();
        let c2 = rng.pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }
        let a3 = rng.pick_1_9();
        let c3 = rng.pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }
        let a4 = rng.pick_1_9();
        let b4 = rng.pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
    std::process::exit(0);
}