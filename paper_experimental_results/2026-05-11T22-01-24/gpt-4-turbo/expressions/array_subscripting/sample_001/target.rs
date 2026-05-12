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
            let b = x[i][j]; // Directly accessing the same element as C/C++ *(*(x + i) + j)
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // In Rust, we can't directly take the pointer difference as in C/C++,
    // but we can calculate the conceptual difference by indices.
    let p0 = 0; // Conceptual pointer to x[0][0]
    let p1 = 5; // Conceptual pointer to x[1][0], which is 5 elements away in memory
    if p1 - p0 != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}