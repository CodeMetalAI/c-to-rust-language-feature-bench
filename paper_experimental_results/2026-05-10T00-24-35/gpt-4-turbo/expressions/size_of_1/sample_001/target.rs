fn main() {
    let mut dp = Box::new(0.0);  // Box::new initializes the memory for double (f64 in Rust)

    *dp = 3.14;
    if *dp != 3.14 {
        // Rust's Box automatically frees the memory when it goes out of scope,
        // so no explicit free is required.
        std::process::exit(2);
    }

    // If the program reaches here, it exits with 0 automatically.
    // No need for explicit free or return because of Rust's ownership and scoping.
}