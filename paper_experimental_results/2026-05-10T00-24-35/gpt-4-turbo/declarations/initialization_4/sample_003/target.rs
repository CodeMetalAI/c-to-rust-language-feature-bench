fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z = [
        [1, 0, 0],
        [2, 0, 0],
        [3, 0, 0],
        [4, 0, 0],
    ];

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

    let p = z.concat();  // Flatten the 2D array into a 1D vector
    if p[1] != 0 {
        std::process::exit(5);
    }
    if p[4] != 2 {
        std::process::exit(6);
    }
    if p[7] != 0 {
        std::process::exit(7);
    }

    std::process::exit(0);
}