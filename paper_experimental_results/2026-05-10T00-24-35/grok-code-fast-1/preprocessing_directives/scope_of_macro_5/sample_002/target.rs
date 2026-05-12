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

const fn build_val_xyz() -> [[[i32; 10]; 10]; 9] {
    let mut arr = [[[0i32; 10]; 10]; 9];
    let mut a = 0;
    while a < 9 {
        let mut b = 0;
        while b < 10 {
            let mut c = 0;
            while c < 10 {
                arr[a][b][c] = ((a as i32 + 1) * 100) + (b as i32 * 10) + (c as i32);
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    arr
}

const VAL_XYZ: [[[i32; 10]; 10]; 9] = build_val_xyz();

const fn build_val_yz() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    let mut b = 0;
    while b < 9 {
        let mut c = 0;
        while c < 10 {
            arr[b][c] = ((b as i32 + 1) * 10) + (c as i32);
            c += 1;
        }
        b += 1;
    }
    arr
}

const VAL_YZ: [[i32; 10]; 9] = build_val_yz();

const fn build_val_xz() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    let mut a = 0;
    while a < 9 {
        let mut c = 0;
        while c < 10 {
            arr[a][c] = ((a as i32 + 1) * 10) + (c as i32);
            c += 1;
        }
        a += 1;
    }
    arr
}

const VAL_XZ: [[i32; 10]; 9] = build_val_xz();

const fn build_val_xy() -> [[i32; 10]; 9] {
    let mut arr = [[0i32; 10]; 9];
    let mut a = 0;
    while a < 9 {
        let mut b = 0;
        while b < 10 {
            arr[a][b] = ((a as i32 + 1) * 10) + (b as i32);
            b += 1;
        }
        a += 1;
    }
    arr
}

const VAL_XY: [[i32; 10]; 9] = build_val_xy();

fn main() {
    let mut rng = Rng::new();
    for _it in 0..50 {
        let a = rng.pick_1_9();
        let b = rng.pick_0_9();
        let c = rng.pick_0_9();
        let v1 = VAL_XYZ[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }
        let b2 = rng.pick_1_9();
        let c2 = rng.pick_0_9();
        let v2 = VAL_YZ[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }
        let a3 = rng.pick_1_9();
        let c3 = rng.pick_0_9();
        let v3 = VAL_XZ[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }
        let a4 = rng.pick_1_9();
        let b4 = rng.pick_0_9();
        let v4 = VAL_XY[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
    std::process::exit(0);
}