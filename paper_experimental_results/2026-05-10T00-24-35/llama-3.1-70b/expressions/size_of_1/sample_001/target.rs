fn main() {
    let dp = Box::new(3.14);

    if dp.is_nan() {
        std::process::exit(2);
    }

    // dp is automatically freed when it goes out of scope
    std::process::exit(0);
}