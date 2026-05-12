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

    let p = |i: usize| -> i32 {
        let row = i / 3;
        let col = i % 3;
        z[row][col]
    };

    if p(1) != 0 {
        exit(5);
    }
    if p(4) != 2 {
        exit(6);
    }
    if p(7) != 0 {
        exit(7);
    }

    exit(0);
}