fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    // Initialize array explicitly to match C behavior (uninitialized elements become 0)
    let mut z = [[0i32; 3]; 4];
    z[0][0] = 1;
    z[1][0] = 2;
    z[2][0] = 3;
    z[3][0] = 4;

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

    // Flatten the 2D array to simulate C's row-major layout
    let flat: &[i32] = &z.into_iter().flatten().collect::<Vec<_>>();
    if flat[1] != 0 {
        std::process::exit(5);
    }
    if flat[4] != 2 {
        std::process::exit(6);
    }
    if flat[7] != 0 {
        std::process::exit(7);
    }

    std::process::exit(0);
}