use rand::Rng;

const MANGLE: fn(i32) -> String = |a: i32| format!("D{}", a);
const MANGLE3: fn(i32, i32, i32) -> String = |a, b, c| format!("{}{}{}", MANGLE(a), MANGLE(b), MANGLE(c));

const PICKD: fn(i32) -> String = |n| MANGLE3(n, n, n);
const VXYZ: fn(i32, i32, i32) -> String = |a, b, c| MANGLE3(PICKD(a), PICKD(b), PICKD(c));
const VYZ: fn(i32, i32) -> String = |b, c| format!("{}{}", PICKD(b), PICKD(c));
const VXZ: fn(i32, i32) -> String = |a, c| format!("{}{}", PICKD(a), PICKD(c));
const VXY: fn(i32, i32) -> String = |a, b| format!("{}{}", PICKD(a), PICKD(b));

static mut RNG_STATE: u32 = 2463534242;

fn pick_1_9() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=9)
}

fn pick_0_9() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10)
}

static VAL_XYZ: [[String; 10]; 9] = [
    [
        VXYZ(1, 1, 0), VXYZ(1, 1, 1), VXYZ(1, 1, 2), VXYZ(1, 1, 3), VXYZ(1, 1, 4),
        VXYZ(1, 1, 5), VXYZ(1, 1, 6), VXYZ(1, 1, 7), VXYZ(1, 1, 8), VXYZ(1, 1, 9),
    ],
    [
        VXYZ(1, 2, 0), VXYZ(1, 2, 1), VXYZ(1, 2, 2), VXYZ(1, 2, 3), VXYZ(1, 2, 4),
        VXYZ(1, 2, 5), VXYZ(1, 2, 6), VXYZ(1, 2, 7), VXYZ(1, 2, 8), VXYZ(1, 2, 9),
    ],
    [
        VXYZ(1, 3, 0), VXYZ(1, 3, 1), VXYZ(1, 3, 2), VXYZ(1, 3, 3), VXYZ(1, 3, 4),
        VXYZ(1, 3, 5), VXYZ(1, 3, 6), VXYZ(1, 3, 7), VXYZ(1, 3, 8), VXYZ(1, 3, 9),
    ],
    [
        VXYZ(1, 4, 0), VXYZ(1, 4, 1), VXYZ(1, 4, 2), VXYZ(1, 4, 3), VXYZ(1, 4, 4),
        VXYZ(1, 4, 5), VXYZ(1, 4, 6), VXYZ(1, 4, 7), VXYZ(1, 4, 8), VXYZ(1, 4, 9),
    ],
    [
        VXYZ(1, 5, 0), VXYZ(1, 5, 1), VXYZ(1, 5, 2), VXYZ(1, 5, 3), VXYZ(1, 5, 4),
        VXYZ(1, 5, 5), VXYZ(1, 5, 6), VXYZ(1, 5, 7), VXYZ(1, 5, 8), VXYZ(1, 5, 9),
    ],
    [
        VXYZ(1, 6, 0), VXYZ(1, 6, 1), VXYZ(1, 6, 2), VXYZ(1, 6, 3), VXYZ(1, 6, 4),
        VXYZ(1, 6, 5), VXYZ(1, 6, 6), VXYZ(1, 6, 7), VXYZ(1, 6, 8), VXYZ(1, 6, 9),
    ],
    [
        VXYZ(1, 7, 0), VXYZ(1, 7, 1), VXYZ(1, 7, 2), VXYZ(1, 7, 3), VXYZ(1, 7, 4),
        VXYZ(1, 7, 5), VXYZ(1, 7, 6), VXYZ(1, 7, 7), VXYZ(1, 7, 8), VXYZ(1, 7, 9),
    ],
    [
        VXYZ(1, 8, 0), VXYZ(1, 8, 1), VXYZ(1, 8, 2), VXYZ(1, 8, 3), VXYZ(1, 8, 4),
        VXYZ(1, 8, 5), VXYZ(1, 8, 6), VXYZ(1, 8, 7), VXYZ(1, 8, 8), VXYZ(1, 8, 9),
    ],
    [
        VXYZ(1, 9, 0), VXYZ(1, 9, 1), VXYZ(1, 9, 2), VXYZ(1, 9, 3), VXYZ(1, 9, 4),
        VXYZ(1, 9, 5), VXYZ(1, 9, 6), VXYZ(1, 9, 7), VXYZ(1, 9, 8), VXYZ(1, 9, 9),
    ],
];

static VAL_YZ: [[String; 10]; 9] = [
    [
        VYZ(1, 0), VYZ(1, 1), VYZ(1, 2), VYZ(1, 3), VYZ(1, 4),
        VYZ(1, 5), VYZ(1, 6), VYZ(1, 7), VYZ(1, 8), VYZ(1, 9),
    ],
    [
        VYZ(2, 0), VYZ(2, 1), VYZ(2, 2), VYZ(2, 3), VYZ(2, 4),
        VYZ(2, 5), VYZ(2, 6), VYZ(2, 7), VYZ(2, 8), VYZ(2, 9),
    ],
    [
        VYZ(3, 0), VYZ(3, 1), VYZ(3, 2), VYZ(3, 3), VYZ(3, 4),
        VYZ(3, 5), VYZ(3, 6), VYZ(3, 7), VYZ(3, 8), VYZ(3, 9),
    ],
    [
        VYZ(4, 0), VYZ(4, 1), VYZ(4, 2), VYZ(4, 3), VYZ(4, 4),
        VYZ(4, 5), VYZ(4, 6), VYZ(4, 7), VYZ(4, 8), VYZ(4, 9),
    ],
    [
        VYZ(5, 0), VYZ(5, 1), VYZ(5, 2), VYZ(5, 3), VYZ(5, 4),
        VYZ(5, 5), VYZ(5, 6), VYZ(5, 7), VYZ(5, 8), VYZ(5, 9),
    ],
    [
        VYZ(6, 0), VYZ(6, 1), VYZ(6, 2), VYZ(6, 3), VYZ(6, 4),
        VYZ(6, 5), VYZ(6, 6), VYZ(6, 7), VYZ(6, 8), VYZ(6, 9),
    ],
    [
        VYZ(7, 0), VYZ(7, 1), VYZ(7, 2), VYZ(7, 3), VYZ(7, 4),
        VYZ(7, 5), VYZ(7, 6), VYZ(7, 7), VYZ(7, 8), VYZ(7, 9),
    ],
    [
        VYZ(8, 0), VYZ(8, 1), VYZ(8, 2), VYZ(8, 3), VYZ(8, 4),
        VYZ(8, 5), VYZ(8, 6), VYZ(8, 7), VYZ(8, 8), VYZ(8, 9),
    ],
    [
        VYZ(9, 0), VYZ(9, 1), VYZ(9, 2), VYZ(9, 3), VYZ(9, 4),
        VYZ(9, 5), VYZ(9, 6), VYZ(9, 7), VYZ(9, 8), VYZ(9, 9),
    ],
];

static VAL_XZ: [[String; 10]; 9] = [
    [
        VXZ(1, 0), VXZ(1, 1), VXZ(1, 2), VXZ(1, 3), VXZ(1, 4),
        VXZ(1, 5), VXZ(1, 6), VXZ(1, 7), VXZ(1, 8), VXZ(1, 9),
    ],
    [
        VXZ(2, 0), VXZ(2, 1), VXZ(2, 2), VXZ(2, 3), VXZ(2, 4),
        VXZ(2, 5), VXZ(2, 6), VXZ(2, 7), VXZ(2, 8), VXZ(2, 9),
    ],
    [
        VXZ(3, 0), VXZ(3, 1), VXZ(3, 2), VXZ(3, 3), VXZ(3, 4),
        VXZ(3, 5), VXZ(3, 6), VXZ(3, 7), VXZ(3, 8), VXZ(3, 9),
    ],
    [
        VXZ(4, 0), VXZ(4, 1), VXZ(4, 2), VXZ(4, 3), VXZ(4, 4),
        VXZ(4, 5), VXZ(4, 6), VXZ(4, 7), VXZ(4, 8), VXZ(4, 9),
    ],
    [
        VXZ(5, 0), VXZ(5, 1), VXZ(5, 2), VXZ(5, 3), VXZ(5, 4),
        VXZ(5, 5), VXZ(5, 6), VXZ(5, 7), VXZ(5, 8), VXZ(5, 9),
    ],
    [
        VXZ(6, 0), VXZ(6, 1), VXZ(6, 2), VXZ(6, 3), VXZ(6, 4),
        VXZ(6, 5), VXZ(6, 6), VXZ(6, 7), VXZ(6, 8), VXZ(6, 9),
    ],
    [
        VXZ(7, 0), VXZ(7, 1), VXZ(7, 2), VXZ(7, 3), VXZ(7, 4),
        VXZ(7, 5), VXZ(7, 6), VXZ(7, 7), VXZ(7, 8), VXZ(7, 9),
    ],
    [
        VXZ(8, 0), VXZ(8, 1), VXZ(8, 2), VXZ(8, 3), VXZ(8, 4),
        VXZ(8, 5), VXZ(8, 6), VXZ(8, 7), VXZ(8, 8), VXZ(8, 9),
    ],
    [
        VXZ(9, 0), VXZ(9, 1), VXZ(9, 2), VXZ(9, 3), VXZ(9, 4),
        VXZ(9, 5), VXZ(9, 6), VXZ(9, 7), VXZ(9, 8), VXZ(9, 9),
    ],
];

static VAL_XY: [[String; 10]; 9] = [
    [
        VXY(1, 0), VXY(1, 1), VXY(1, 2), VXY(1, 3), VXY(1, 4),
        VXY(1, 5), VXY(1, 6), VXY(1, 7), VXY(1, 8), VXY(1, 9),
    ],
    [
        VXY(2, 0), VXY(2, 1), VXY(2, 2), VXY(2, 3), VXY(2, 4),
        VXY(2, 5), VXY(2, 6), VXY(2, 7), VXY(2, 8), VXY(2, 9),
    ],
    [
        VXY(3, 0), VXY(3, 1), VXY(3, 2), VXY(3, 3), VXY(3, 4),
        VXY(3, 5), VXY(3, 6), VXY(3, 7), VXY(3, 8), VXY(3, 9),
    ],
    [
        VXY(4, 0), VXY(4, 1), VXY(4, 2), VXY(4, 3), VXY(4, 4),
        VXY(4, 5), VXY(4, 6), VXY(4, 7), VXY(4, 8), VXY(4, 9),
    ],
    [
        VXY(5, 0), VXY(5, 1), VXY(5, 2), VXY(5, 3), VXY(5, 4),
        VXY(5, 5), VXY(5, 6), VXY(5, 7), VXY(5, 8), VXY(5, 9),
    ],
    [
        VXY(6, 0), VXY(6, 1), VXY(6, 2), VXY(6, 3), VXY(6, 4),
        VXY(6, 5), VXY(6, 6), VXY(6, 7), VXY(6, 8), VXY(6, 9),
    ],
    [
        VXY(7, 0), VXY(7, 1), VXY(7, 2), VXY(7, 3), VXY(7, 4),
        VXY(7, 5), VXY(7, 6), VXY(7, 7), VXY(7, 8), VXY(7, 9),
    ],
    [
        VXY(8, 0), VXY(8, 1), VXY(8, 2), VXY(8, 3), VXY(8, 4),
        VXY(8, 5), VXY(8, 6), VXY(8, 7), VXY(8, 8), VXY(8, 9),
    ],
    [
        VXY(9, 0), VXY(9, 1), VXY(9, 2), VXY(9, 3), VXY(9, 4),
        VXY(9, 5), VXY(9, 6), VXY(9, 7), VXY(9, 8), VXY(9, 9),
    ],
];

fn test_xy() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = VAL_XYZ[a - 1][b][c].parse::<i32>().unwrap();
        if v1 != 100 * a + 10 * b + c {
            panic!("test xy failed");
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = VAL_YZ[b2 - 1][c2].parse::<i32>().unwrap();
        if v2 != 10 * b2 + c2 {
            panic!("test yz failed");
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = VAL_XZ[a3 - 1][c3].parse::<i32>().unwrap();
        if v3 != 10 * a3 + c3 {
            panic!("test xz failed");
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = VAL_XY[a4 - 1][b4].parse::<i32>().unwrap();
        if v4 != 10 * a4 + b4 {
            panic!("test xy failed");
        }
    }
}

fn main() {
    test_xy();
    println!("success");
}