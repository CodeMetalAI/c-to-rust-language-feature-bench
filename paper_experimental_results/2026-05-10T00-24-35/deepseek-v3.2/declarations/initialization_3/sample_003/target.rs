fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
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
        let p1 = &y1[0][0];
        let p2 = &y2[0][0];
        
        // Access elements using pointer arithmetic simulation via flat indexing
        let flat_index_11 = 11;
        if flat_index_11 < 12 { // 4 * 3 = 12 total elements
            // In C, p1[11] is equivalent to *(p1 + 11), which is y1[3][2] in row-major order.
            // Since y1 is initialized with explicit zeros in the last row, y1[3][2] should be 0.
            // We'll access it safely using array indexing.
            let row = flat_index_11 / 3;
            let col = flat_index_11 % 3;
            if y1[row][col] != 0 {
                std::process::exit(9);
            }
            if y2[row][col] != 0 {
                std::process::exit(10);
            }
        }
    }

    std::process::exit(0);
}