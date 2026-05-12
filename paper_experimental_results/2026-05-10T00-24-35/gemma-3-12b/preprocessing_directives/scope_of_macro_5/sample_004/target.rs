#[macro_export]
macro_rules! CAT {
    ($a:expr, $b:expr) => {
        concat!($a, $b)
    };
}

#[macro_export]
macro_rules! XCAT {
    ($a:expr, $b:expr) => {
        CAT!($a, $b)
    };
}

#[macro_export]
macro_rules! CAT3 {
    ($a:expr, $b:expr, $c:expr) => {
        XCAT!($a, XCAT!($b, $c))
    };
}

#[macro_export]
macro_rules! PICKD {
    ($n:expr) => {
        concat!("D", $n)
    };
}

#[macro_export]
macro_rules! VXYZ {
    ($a:expr, $b:expr, $c:expr) => {
        XCAT!(PICKD!($a), XCAT!(PICKD!($b), PICKD!($c)))
    };
}

#[macro_export]
macro_rules! VYZ {
    ($b:expr, $c:expr) => {
        XCAT!(PICKD!($b), PICKD!($c))
    };
}

#[macro_export]
macro_rules! VXZ {
    ($a:expr, $c:expr) => {
        XCAT!(PICKD!($a), PICKD!($c))
    };
}

#[macro_export]
macro_rules! VXY {
    ($a:expr, $b:expr) => {
        XCAT!(PICKD!($a), PICKD!($b))
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

static VAL_XYZ: [[[[i32; 10]; 10]; 9] as *const [[i32; 10]; 10]; 9] = {
    let mut arr = [[[0; 10]; 10]; 9];
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

        let v1 = unsafe { VAL_XYZ[a - 1][b][c] };
        if v1 != 100 * a + 10 * b + c {
            return 1;
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = unsafe { VAL_YZ[b2 - 1][c2] };
        if v2 != 10 * b2 + c2 {
            return 2;
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = unsafe { VAL_XZ[a3 - 1][c3] };
        if v3 != 10 * a3 + c3 {
            return 3;
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = unsafe { VAL_XY[a4 - 1][b4] };
        if v4 != 10 * a4 + b4 {
            return 4;
        }
    }

    0
}