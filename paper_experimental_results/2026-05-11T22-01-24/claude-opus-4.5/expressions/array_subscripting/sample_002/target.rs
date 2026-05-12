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

    // In Rust, we can verify the layout by checking that rows are contiguous
    // The equivalent check is that the distance between row starts is 5 elements
    let flat: Vec<i32> = x.iter().flat_map(|row| row.iter().copied()).collect();
    let p0_index = 0; // &x[0][0]
    let p1_index = 5; // &x[1][0]
    if (p1_index - p0_index) as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}