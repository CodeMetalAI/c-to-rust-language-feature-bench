fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In Rust, chained comparison like `a < b < c` is not allowed directly.
    // We need to use logical AND to combine two comparisons.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // In Rust, `(a < b) < c` does not make sense as `(a < b)` is a boolean.
    // Comparing a boolean to an integer is not valid in Rust.
    // The equivalent C++ behavior is that `(a < b)` is evaluated to `1` (true) or `0` (false),
    // and then it is compared to `c`. We simulate this by converting the boolean to an integer.
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