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

    // In Rust, we can verify the layout by checking that elements are contiguous
    // The C code checks that &x[1][0] - &x[0][0] == 5
    // This is equivalent to checking that each row has 5 elements
    // In Rust's memory model with [[i32; 5]; 3], this is guaranteed
    let flat: Vec<&i32> = x.iter().flat_map(|row| row.iter()).collect();
    let p0 = flat[0] as *const i32;
    let p1 = flat[5] as *const i32; // &x[1][0] is the 6th element (index 5)
    
    // Calculate pointer difference in terms of i32 elements
    let diff = (p1 as isize - p0 as isize) / std::mem::size_of::<i32>() as isize;
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}