fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() -> i32 {
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
        return 1;
    }
    if !check_row(&y1[1], 2, 4, 6) {
        return 2;
    }
    if !check_row(&y1[2], 3, 5, 7) {
        return 3;
    }
    if !check_row(&y1[3], 0, 0, 0) {
        return 4;
    }

    if !check_row(&y2[0], 1, 3, 5) {
        return 5;
    }
    if !check_row(&y2[1], 2, 4, 6) {
        return 6;
    }
    if !check_row(&y2[2], 3, 5, 7) {
        return 7;
    }
    if !check_row(&y2[3], 0, 0, 0) {
        return 8;
    }

    let p1 = &y1[0][0] as *const i32 as *const i32;
    let p2 = &y2[0][0] as *const i32 as *const i32;
    if unsafe { *p1.add(11) } != 0 {
        return 9;
    }
    if unsafe { *p2.add(11) } != 0 {
        return 10;
    }

    return 0;
}