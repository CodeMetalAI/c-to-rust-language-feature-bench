fn check_row(a: &[i32], x: i32, y: i32, z: i32) -> bool {
    a.len() >= 3 && a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [i32; 12] = [1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0];

    if !check_row(&z[0..3], 1, 0, 0) {
        std::process::exit(1);
    }
    if !check_row(&z[3..6], 2, 0, 0) {
        std::process::exit(2);
    }
    if !check_row(&z[6..9], 3, 0, 0) {
        std::process::exit(3);
    }
    if !check_row(&z[9..12], 4, 0, 0) {
        std::process::exit(4);
    }

    {
        let p: &[i32; 12] = &z;
        if p[1] != 0 {
            std::process::exit(5);
        }
        if p[4] != 2 {
            std::process::exit(6);
        }
        if p[7] != 0 {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}