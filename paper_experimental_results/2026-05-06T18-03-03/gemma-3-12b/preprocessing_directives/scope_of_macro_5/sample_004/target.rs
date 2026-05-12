#[macro_use]
macro_rules! cat {
    ($a:expr, $b:expr) => {
        concat!($a, $b)
    };
}

#[macro_use]
macro_rules! xcat {
    ($a:expr, $b:expr) => {
        cat!($a, $b)
    };
}

#[macro_use]
macro_rules! cat3 {
    ($a:expr, $b:expr, $c:expr) => {
        xcat!($a, $b, $c)
    };
}

macro_rules! pickd {
    ($n:expr) => {
        concat!("D", $n)
    };
}

macro_rules! vxyz {
    ($a:expr, $b:expr, $c:expr) => {
        cat3!(pickd!($a), pickd!($b), pickd!($c))
    };
}

macro_rules! vyz {
    ($b:expr, $c:expr) => {
        xcat!(pickd!($b), pickd!($c))
    };
}

macro_rules! vxz {
    ($a:expr, $c:expr) => {
        xcat!(pickd!($a), pickd!($c))
    };
}

macro_rules! vxy {
    ($a:expr, $b:expr) => {
        xcat!(pickd!($a), pickd!($b))
    };
}

static mut rng_state: u32 = 2463534242;

fn rng_next() -> u32 {
    unsafe {
        let mut x = rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng_state = x;
        x
    }
}

fn pick_1_9() -> i32 {
    (rng_next() % 9) as i32 + 1
}

fn pick_0_9() -> i32 {
    (rng_next() % 10) as i32
}

static VAL_XYZ: [[[[i32; 10]; 10]; 9]; 9] = {
    let mut arr = [[[[0; 10]; 10]; 9]; 9];
    for i in 0..9 {
        for j in 0..10 {
            for k in 0..10 {
                arr[i][j][k] = 100 * (i + 1) as i32 + 10 * j as i32 + k as i32;
            }
        }
    }
    arr
};

static VAL_YZ: [[i32; 10]; 9] = {
    let mut arr = [[0; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i + 1) as i32 + j as i32;
        }
    }
    arr
};

static VAL_XZ: [[i32; 10]; 9] = {
    let mut arr = [[0; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i + 1) as i32 + j as i32;
        }
    }
    arr
};

static VAL_XY: [[i32; 10]; 9] = {
    let mut arr = [[0; 10]; 9];
    for i in 0..9 {
        for j in 0..10 {
            arr[i][j] = 10 * (i + 1) as i32 + j as i32;
        }
    }
    arr
};

fn main() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = VAL_XYZ[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}