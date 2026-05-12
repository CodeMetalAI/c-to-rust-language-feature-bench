use std::process::exit;

fn main() {
    // Allocate a double on the heap
    let mut dp = Box::new(0.0f64);

    // In Rust, allocation failures typically abort the process,
    // so we proceed assuming success.
    *dp = 3.14;
    if *dp != 3.14 {
        exit(2);
    }

    exit(0);
}