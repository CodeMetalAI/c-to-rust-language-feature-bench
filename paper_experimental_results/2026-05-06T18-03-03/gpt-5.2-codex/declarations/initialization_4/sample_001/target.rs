use std::process::exit;

fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn get_flat(z: &[[i32; 3]; 4], idx: usize) -> i32 {
    let row = idx / 3;
    let col = idx % 3;
    z[row][col]
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

    if get_flat(&z, 1) != 0 {
        exit(5);
    }
    if get_flat(&z, 4) != 2 {
        exit(6);
    }
    if get_flat(&z, 7) != 0 {
        exit(7);
    }

    exit(0);
}