fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In Rust, chained comparisons like `a < b < c` are not allowed.
    // We need to explicitly separate them into two comparisons.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // The expression `(a < b) < c` in C++ is equivalent to comparing the result of `a < b` (which is a bool) with `c`.
    // In Rust, you cannot directly compare a bool with an integer, so we convert the bool to an integer.
    if !(((a < b) as i32) < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !(a < b && b < c) {
        std::process::exit(3);
    }

    // If all conditions pass, return 0.
    // In Rust, returning 0 from `main` is implicit when no error occurs.
}