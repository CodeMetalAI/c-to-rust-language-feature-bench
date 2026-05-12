/*
 * size_of_3
 */

fn fsize3(n: usize) -> usize {
    let b = vec![0u8; n + 3]; // Create a vector with n + 3 elements
    std::mem::size_of_val(&b) // Return the size of the vector
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}