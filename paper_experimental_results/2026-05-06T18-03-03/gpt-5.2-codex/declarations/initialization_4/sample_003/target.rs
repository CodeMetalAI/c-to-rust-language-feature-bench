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

    let mut code = 0;

    if !check_row(&z[0], 1, 0, 0) {
        code = 1;
    } else if !check_row(&z[1], 2, 0, 0) {
        code = 2;
    } else if !check_row(&z[2], 3, 0, 0) {
        code = 3;
    } else if !check_row(&z[3], 4, 0, 0) {
        code = 4;
    } else {
        let get = |idx: usize| -> i32 {
            let row = idx / 3;
            let col = idx % 3;
            z[row][col]
        };

        if get(1) != 0 {
            code = 5;
        } else if get(4) != 2 {
            code = 6;
        } else if get(7) != 0 {
            code = 7;
        }
    }

    std::process::exit(code);
}