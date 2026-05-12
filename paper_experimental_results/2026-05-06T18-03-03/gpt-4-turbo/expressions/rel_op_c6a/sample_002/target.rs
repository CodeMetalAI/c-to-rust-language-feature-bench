fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    // In Rust, chained comparison like `a < b < c` is not allowed directly.
    // We need to use logical AND to combine two comparisons.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // The expression `(a < b) < c` in C++ is equivalent to comparing the boolean result of `a < b` to `c`.
    // In Rust, we need to explicitly convert the boolean to integer to make a similar comparison.
    if !((a < b) as i32 < c) {
        std::process::exit(2);
    }

    a = 3;

    // Rechecking with new values of `a`, `b`, and `c`.
    if !(a < b && b < c) {
        std::process::exit(3);
    }

    // If all conditions are passed, return 0.
}