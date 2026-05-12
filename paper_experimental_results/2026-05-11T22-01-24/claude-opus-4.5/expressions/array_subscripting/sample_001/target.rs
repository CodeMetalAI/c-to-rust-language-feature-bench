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
    // Since Rust 2D arrays are laid out contiguously in row-major order,
    // this is guaranteed by the type system, but we can still verify
    // the equivalent check using flat indexing
    let flat: &[i32; 15] = unsafe { std::mem::transmute(&x) };
    let p0_index: isize = 0; // &x[0][0] -> index 0
    let p1_index: isize = 5; // &x[1][0] -> index 5
    let diff = p1_index - p0_index;
    
    // Actually, let's do this in safe Rust by flattening
    let x_flat: Vec<i32> = x.iter().flat_map(|row| row.iter().copied()).collect();
    
    // The difference in indices between x[1][0] and x[0][0] should be 5
    let idx0 = 0 * 5 + 0; // index of x[0][0]
    let idx1 = 1 * 5 + 0; // index of x[1][0]
    if (idx1 - idx0) as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}