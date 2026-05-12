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
        XCAT!($a, $b)
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
        XCAT!(PICKD!($a), PICKD!($b), PICKD!($c))
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

static VAL_XYZ: [[[[i32; 10]; 10]; 9] as '_];
static VAL_YZ: [[i32; 10]; 9] as '_;
static VAL_XZ: [[i32; 10]; 9] as '_;
static VAL_XY: [[i32; 10]; 9] as '_;

const VAL_XYZ: [[[[i32; 10]; 10]; 9]; 9]; 10]; 10] = {
    let mut array = [[[[0; 10]; 10]; 9]; 9];
    for a in 1..=9 {
        for b in 0..10 {
            for c in 0..10 {
                array[a - 1][b][c] = 100 * a as i32 + 10 * b as i32 + c as i32;
            }
        }
    }
    array
};

const VAL_YZ: [[i32; 10]; 9] = {
    let mut array = [[0; 10]; 9];
    for b in 1..=9 {
        for c in 0..10 {
            array[b - 1][c] = 10 * b as i32 + c as i32;
        }
    }
    array
};

const VAL_XZ: [[i32; 10]; 9] = {
    let mut array = [[0; 10]; 9];
    for a in 1..=9 {
        for c in 0..10 {
            array[a - 1][c] = 10 * a as i32 + c as i32;
        }
    }
    array
};

const VAL_XY: [[i32; 10]; 9] = {
    let mut array = [[0; 10]; 9];
    for a in 1..=9 {
        for b in 0..10 {
            array[a - 1][b] = 10 * a as i32 + b as i32;
        }
    }
    array
};

fn main() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = VAL_XYZ[a - 1][b][c];
        if v1 != 100 * a as i32 + 10 * b as i32 + c as i32 {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[b2 - 1][c2];
        if v2 != 10 * b2 as i32 + c2 as i32 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[a3 - 1][c3];
        if v3 != 10 * a3 as i32 + c3 as i32 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[a4 - 1][b4];
        if v4 != 10 * a4 as i32 + b4 as i32 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}