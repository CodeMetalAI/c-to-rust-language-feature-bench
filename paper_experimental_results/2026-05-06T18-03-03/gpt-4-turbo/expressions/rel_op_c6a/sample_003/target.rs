fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In Rust, chained comparison like `a < b < c` is not allowed directly.
    // We need to explicitly separate them into two comparisons.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // The expression `(a < b) < c` in C++ is equivalent to comparing the boolean result of `a < b` with `c`.
    // In Rust, we need to convert the boolean to an integer to make a similar comparison.
    if !((a < b) as i32 < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !(a < b && b < c) {
        std::process::exit(3);
    }

    // If all conditions pass, return 0.
}