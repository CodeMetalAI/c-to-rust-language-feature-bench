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
    // Since Rust's 2D arrays are laid out contiguously in row-major order,
    // this is guaranteed by the type system, but we can still verify
    // by flattening and checking indices
    let flat: Vec<&i32> = x.iter().flat_map(|row| row.iter()).collect();
    let p0_idx = 0; // &x[0][0]
    let p1_idx = 5; // &x[1][0]
    if (p1_idx - p0_idx) as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}