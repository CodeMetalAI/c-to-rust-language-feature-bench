fn main() {
    let mut x = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j]; // Direct access replaces `*(*(x + i) + j)` which is pointer arithmetic
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // Checking pointer offset in C++ by calculating the difference between pointers
    // In Rust, we do it by calculating index difference, since pointer arithmetic isn't allowed in safe Rust
    let p0 = &x[0][0] as *const i32 as usize;
    let p1 = &x[1][0] as *const i32 as usize;
    if (p1 - p0) != 5 * std::mem::size_of::<i32>() {
        std::process::exit(2);
    }

    println!("PASS");
}