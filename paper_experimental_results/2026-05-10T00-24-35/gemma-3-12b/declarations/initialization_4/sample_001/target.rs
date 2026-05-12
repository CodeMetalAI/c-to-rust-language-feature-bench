fn check_row(a: &[int; 3], x: int, y: int, z: int) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[int; 3]; 4] = [[1], [2], [3], [4]];

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
        let mut p: Vec<int> = z[0].iter().cloned().collect();
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

    0
}