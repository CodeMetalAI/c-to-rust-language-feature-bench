fn main() {
    let mut dp = Box::new(0.0); // allocate memory for a double
    *dp = 3.14; // assign 3.14 to the allocated memory

    if *dp != 3.14 {
        std::process::exit(2); // exit with code 2 if value is not 3.14
    }

    // Box is automatically freed when it goes out of scope
}