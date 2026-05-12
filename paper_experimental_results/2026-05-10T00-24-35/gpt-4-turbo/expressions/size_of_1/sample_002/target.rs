fn main() {
    let mut dp = Box::new(0.0f64);  // dynamically allocate space for a double (f64 in Rust)

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    // Box automatically freed at the end of scope
}