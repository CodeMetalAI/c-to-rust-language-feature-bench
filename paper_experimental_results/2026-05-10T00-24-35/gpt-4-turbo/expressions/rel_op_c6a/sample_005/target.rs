fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    // In Rust, `a < b < c` is not valid. We must separate comparisons.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // This is the intended translation, but Rust also does not support `(a < b) < c`.
    // Instead, `(a < b)` evaluates to a boolean, which cannot be compared with `c`.
    if !((a < b) && (b < c)) {
        std::process::exit(2);
    }

    a = 3;
    let b = 2;
    let c = 2;

    if !(a < b && b < c) {
        std::process::exit(3);
    }

    // If all checks pass, return 0.
}