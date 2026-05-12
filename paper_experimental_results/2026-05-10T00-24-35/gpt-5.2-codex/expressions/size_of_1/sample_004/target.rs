fn main() {
    let mut dp = Box::new(0.0f64);

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    std::process::exit(0);
}