fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z = [
        [1, 0, 0],
        [2, 0, 0],
        [3, 0, 0],
        [4, 0, 0]
    ];

    if !check_row(&z[0], 1, 0, 0) {
        return std::process::exit(1);
    }
    if !check_row(&z[1], 2, 0, 0) {
        return std::process::exit(2);
    }
    if !check_row(&z[2], 3, 0, 0) {
        return std::process::exit(3);
    }
    if !check_row(&z[3], 4, 0, 0) {
        return std::process::exit(4);
    }

    let p = &z[0] as *const _ as *const i32; // mimic pointer to first element
    unsafe { // Safe Rust required, but this is only a safe usage mimic from C++
        if *p.add(1) != 0 {
            return std::process::exit(5);
        }
        if *p.add(4) != 2 {
            return std::process::exit(6);
        }
        if *p.add(7) != 0 {
            return std::process::exit(7);
        }
    }

    std::process::exit(0);
}