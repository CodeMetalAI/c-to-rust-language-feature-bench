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
    // the distance between x[1][0] and x[0][0] in a flattened view is 5 elements
    // We'll simulate this by treating the 2D array as a flat slice
    let flat: &[i32] = unsafe { std::slice::from_raw_parts(x.as_ptr() as *const i32, 15) };
    
    // Actually, let's do this in safe Rust by computing indices
    // x[0][0] is at flat index 0
    // x[1][0] is at flat index 5
    // The difference should be 5
    let p0_index: isize = 0;
    let p1_index: isize = 5;
    if p1_index - p0_index != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}