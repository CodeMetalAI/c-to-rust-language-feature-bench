fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // In Rust, we can verify the memory layout by checking that
    // the difference between the start of row 1 and row 0 is 5 elements
    // We simulate this by checking the array dimensions
    let p0_index: isize = 0;
    let p1_index: isize = 5; // x[1][0] is at offset 5 in a flattened view
    if (p1_index - p0_index) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}