const fn compute_val_xyz() -> [[[i32; 10]; 10]; 9] {
    let mut arr = [[[0; 10]; 10]; 9];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 10 {
            let mut k = 0;
            while k < 10 {
                arr[i][j][k] = ((i + 1) as i32) * 100 + (j as i32) * 10 + (k as i32);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    arr
}

const fn compute_val_yz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 10 {
            arr[i][j] = ((i + 1) as i32) * 10 + (j as i32);
            j += 1;
        }
        i += 1;
    }
    arr
}

const fn compute_val_xz() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 10 {
            arr[i][j] = ((i + 1) as i32) * 10 + (j as i32);
            j += 1;
        }
        i += 1;
    }
    arr
}

const fn compute_val_xy() -> [[i32; 10]; 9] {
    let mut arr = [[0; 10]; 9];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 10 {
            arr[i][j] = ((i + 1) as i32) * 10 + (j as i32);
            j += 1;
        }
        i += 1;
    }
    arr
}

static VAL_XYZ: [[[i32; 10]; 10]; 9] = compute_val_xyz();
static VAL_YZ: [[i32; 10]; 9] = compute_val_yz();
static VAL_XZ: [[i32; 10]; 9] = compute_val_xz();
static VAL_XY: [[i32; 10]; 9] = compute_val_xy();

struct Rng {
    state: u32,
}

impl Rng {
    fn new() -> Self {
        Rng { state: 2463534242 }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

fn pick_1_9(rng: &mut Rng) -> i32 {
    (rng.next() % 9) as i32 + 1
}

fn pick_0_9(rng: &mut Rng) -> i32 {
    (rng.next() % 10) as i32
}

fn main() {
    let mut rng = Rng::new();
    for _ in 0..50 {
        let a = pick_1_9(&mut rng);
        let b = pick_0_9(&mut rng);
        let c = pick_0_9(&mut rng);

        let v1 = VAL_XYZ[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng);
        let c2 = pick_0_9(&mut rng);
        let v2 = VAL_YZ[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng);
        let c3 = pick_0_9(&mut rng);
        let v3 = VAL_XZ[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng);
        let b4 = pick_0_9(&mut rng);
        let v4 = VAL_XY[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
    std::process::exit(0);
}