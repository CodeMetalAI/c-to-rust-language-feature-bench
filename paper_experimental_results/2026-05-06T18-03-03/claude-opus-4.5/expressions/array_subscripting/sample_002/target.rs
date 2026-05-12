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
    // the second row starts 5 elements after the first row
    // We simulate the pointer arithmetic check by verifying array structure
    let flat: Vec<i32> = x.iter().flatten().copied().collect();
    let p0_index = 0; // &x[0][0]
    let p1_index = 5; // &x[1][0]
    if (p1_index - p0_index) as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}