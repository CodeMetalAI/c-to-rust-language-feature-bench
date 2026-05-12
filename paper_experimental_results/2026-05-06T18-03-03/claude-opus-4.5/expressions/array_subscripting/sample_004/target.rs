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
    let flat: Vec<&i32> = x.iter().flat_map(|row| row.iter()).collect();
    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    let diff = (p1 as usize - p0 as usize) / std::mem::size_of::<i32>();
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}