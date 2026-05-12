fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[i32; 3]; 4] = [[1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]];

    if !check_row(&z[0], 1, 0, 0) {
        std::process::exit(1);
    }
    if !check_row(&z[1], 2, 0, 0) {
        std::process::exit(2);
    }
    if !check_row(&z[2], 3, 0, 0) {
        std::process::exit(3);
    }
    if !check_row(&z[3], 4, 0, 0) {
        std::process::exit(4);
    }

    // Simulate pointer accesses
    let p1_row = (1 / 3) as usize;
    let p1_col = (1 % 3) as usize;
    if z[p1_row][p1_col] != 0 {
        std::process::exit(5);
    }
    let p4_row = (4 / 3) as usize;
    let p4_col = (4 % 3) as usize;
    if z[p4_row][p4_col] != 2 {
        std::process::exit(6);
    }
    let p7_row = (7 / 3) as usize;
    let p7_col = (7 % 3) as usize;
    if z[p7_row][p7_col] != 0 {
        std::process::exit(7);
    }

    std::process::exit(0);
}