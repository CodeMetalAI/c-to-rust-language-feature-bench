use std::process::exit;

fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[i32; 3]; 4] = [[1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]];

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

    let flat: [i32; 12] = [
        z[0][0], z[0][1], z[0][2],
        z[1][0], z[1][1], z[1][2],
        z[2][0], z[2][1], z[2][2],
        z[3][0], z[3][1], z[3][2],
    ];

    if flat[1] != 0 {
        exit(5);
    }
    if flat[4] != 2 {
        exit(6);
    }
    if flat[7] != 0 {
        exit(7);
    }

    exit(0);
}