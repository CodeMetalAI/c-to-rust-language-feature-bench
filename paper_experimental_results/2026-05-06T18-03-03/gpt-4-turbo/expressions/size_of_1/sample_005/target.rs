fn main() {
    let mut dp = Box::new(0.0f64); // Allocate space for a double (f64 in Rust)

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    // Box is automatically freed when it goes out of scope
    std::process::exit(0);
}