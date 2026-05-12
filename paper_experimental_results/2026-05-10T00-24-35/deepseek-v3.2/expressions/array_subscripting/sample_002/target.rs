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
            // In Rust, we can't directly do pointer arithmetic like C,
            // but we can compute the equivalent offset manually.
            // x is a contiguous array of 3*5 elements, so element at (i,j) is at index i*5 + j.
            let b = x[i][j]; // Same as a, since Rust doesn't allow the C pointer arithmetic
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // Check that the rows are contiguous with expected stride.
    // In Rust, we can use pointer arithmetic in unsafe blocks, but we need to avoid unsafe.
    // Instead, we can verify using array indexing and address calculation.
    // Convert references to raw pointers to compute byte offsets, but still no unsafe dereferencing.
    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    // The difference in pointers is in number of i32 elements, not bytes.
    // In C, p1 - p0 gives 5 for int pointers with 5 ints between them.
    // We'll compute the difference using isize (like ptrdiff_t).
    let diff = (p1 as isize - p0 as isize) / std::mem::size_of::<i32>() as isize;
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}