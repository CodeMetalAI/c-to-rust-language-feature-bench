fn check_row(row: &[i32], x: i32, y: i32, z: i32) -> bool {
    row[0] == x && row[1] == y && row[2] == z
}

fn main() {
    let y1: [i32; 12] = [1, 3, 5, 2, 4, 6, 3, 5, 7, 0, 0, 0];
    let y2: [i32; 12] = [1, 3, 5, 2, 4, 6, 3, 5, 7, 0, 0, 0];

    if !check_row(&y1[0..3], 1, 3, 5) {
        std::process::exit(1);
    }
    if !check_row(&y1[3..6], 2, 4, 6) {
        std::process::exit(2);
    }
    if !check_row(&y1[6..9], 3, 5, 7) {
        std::process::exit(3);
    }
    if !check_row(&y1[9..12], 0, 0, 0) {
        std::process::exit(4);
    }

    if !check_row(&y2[0..3], 1, 3, 5) {
        std::process::exit(5);
    }
    if !check_row(&y2[3..6], 2, 4, 6) {
        std::process::exit(6);
    }
    if !check_row(&y2[6..9], 3, 5, 7) {
        std::process::exit(7);
    }
    if !check_row(&y2[9..12], 0, 0, 0) {
        std::process::exit(8);
    }

    {
        let p1: &[i32] = &y1;
        let p2: &[i32] = &y2;
        if p1[11] != 0 {
            std::process::exit(9);
        }
        if p2[11] != 0 {
            std::process::exit(10);
        }
    }

    std::process::exit(0);
}