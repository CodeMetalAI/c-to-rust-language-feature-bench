use std::process::exit;

fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let y1: [[i32; 3]; 4] = [
        [1, 3, 5],
        [2, 4, 6],
        [3, 5, 7],
        [0, 0, 0],
    ];

    let y2: [[i32; 3]; 4] = [
        [1, 3, 5],
        [2, 4, 6],
        [3, 5, 7],
        [0, 0, 0],
    ];

    if !check_row(&y1[0], 1, 3, 5) {
        exit(1);
    }
    if !check_row(&y1[1], 2, 4, 6) {
        exit(2);
    }
    if !check_row(&y1[2], 3, 5, 7) {
        exit(3);
    }
    if !check_row(&y1[3], 0, 0, 0) {
        exit(4);
    }

    if !check_row(&y2[0], 1, 3, 5) {
        exit(5);
    }
    if !check_row(&y2[1], 2, 4, 6) {
        exit(6);
    }
    if !check_row(&y2[2], 3, 5, 7) {
        exit(7);
    }
    if !check_row(&y2[3], 0, 0, 0) {
        exit(8);
    }

    {
        let p1_val = y1[3][2]; // equivalent to p1[11]
        let p2_val = y2[3][2]; // equivalent to p2[11]
        if p1_val != 0 {
            exit(9);
        }
        if p2_val != 0 {
            exit(10);
        }
    }

    exit(0);
}