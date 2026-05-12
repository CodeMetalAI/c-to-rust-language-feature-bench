fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let mut z = [[1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]];

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

    {
        let p = &z[0][0];
        if p + 1 != &z[0][1] {
            // In C, p[1] is pointer arithmetic: *(p + 1)
            // In Rust, we need to calculate the offset manually
            let offset_1 = unsafe { *(p as *const i32).offset(1) };
            if offset_1 != 0 {
                std::process::exit(5);
            }
        }
        let offset_4 = unsafe { *(p as *const i32).offset(4) };
        if offset_4 != 2 {
            std::process::exit(6);
        }
        let offset_7 = unsafe { *(p as *const i32).offset(7) };
        if offset_7 != 0 {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}