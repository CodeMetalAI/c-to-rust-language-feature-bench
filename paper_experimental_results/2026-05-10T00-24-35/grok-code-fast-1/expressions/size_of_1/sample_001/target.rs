fn main() {
    let dp = Box::new(0.0);

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    std::process::exit(0);
}