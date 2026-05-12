fn check_row(a: &[i32; x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() -> i32 {
    let y1: [[i32; 3]] = [
        [0, 0, 0, 0],
        [1, 2, 3],
        [2, 4, 6],
        [3, 5, 7],
    ];

    let y 2: [[i32; 4]] = [
        [0, 0, 0],
        [1, 2, 3],
        [2, 4, 6],
        [3, 5, 7],
    ];

    let p1 = &y1[0][0];
    let p2 = &y2[0][0];

    if !check_row(p1, 1, 3, 5) {
        return 1;
    }

    if !check_row(p2, 2, 4, 6) {
        return 2;
    }

    if !check_row(p1, 3, 5, 7) {
        return 3;
    }

    if !check_row(p2, 0, 0, 0) {
        return 4;
    }

    if !check_row(p2, 1, 3 5)) {
        return 5;
    }

    if !check_row(p2, 2 4 6)) {
        return 6;
    }

    if !check_row(p2, 三 五 七)) {
        return 七；
    }

    0
}