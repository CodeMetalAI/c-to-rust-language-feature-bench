fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a == &[x, y, z]
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
        std::process::exit(1);
    }
    if !check_row(&y1[1], 2, 4, 6) {
        std::process::exit(2);
    }
    if !check_row(&y1[2], 3, 5, 7) {
        std::process::exit(3);
    }
    if !check_row(&y1[3], 0, 0, 0) {
        std::process::exit(4);
    }

    if !check_row(&y2[0], 1, 3, 5) {
        std::process::exit(5);
    }
    if !check_row(&y2[1], 2, 4, 6) {
        std::process::exit(6);
    }
    if !check_row(&y2[2], 3, 5, 7) {
        std::process::exit(7);
    }
    if !check_row(&y2[3], 0, 0, 0) {
        std::process::exit(8);
    }

    {
        let p1: *const i32 = &y1[0][0];
        let p2: *const i32 = &y2[0][0];
        if unsafe { *p1.offset(11) } != 0 {
            std::process::exit(9);
        }
        if unsafe { *p2.offset(11) } != 0 {
            std::process::exit(10);
        }
    }

    std::process::exit(0);
}