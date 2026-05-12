use std::process::exit;

fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[i32; 3]; 4] = [
        [1, 0, 0],
        [2, 0, 0],
        [3, 0, 0],
        [4, 0, 0],
    ];

    if !check_row(&z[0], 1, 0, 0) {
        exit(1);
    }
    if !check_row(&z[1], 2, 0, 0) {
        exit(2);
    }
    if !check_row(&z[2], 3, 0, 0) {
        exit(3);
    }
    if !check_row(&z[3], 4, 0, 0) {
        exit(4);
    }

    let p1 = z[1 / 3][1 % 3];
    if p1 != 0 {
        exit(5);
    }
    let p4 = z[4 / 3][4 % 3];
    if p4 != 2 {
        exit(6);
    }
    let p7 = z[7 / 3][7 % 3];
    if p7 != 0 {
        exit(7);
    }

    exit(0);
}