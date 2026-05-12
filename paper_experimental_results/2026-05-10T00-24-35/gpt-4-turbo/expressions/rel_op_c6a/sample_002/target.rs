fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    // Translating `a < b < c` which is not directly valid in Rust
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // Translating `((a < b) < c)` which is also incorrect in Rust due to type mismatch
    // The expression `(a < b)` is a boolean, and `bool` cannot be compared with `int` directly.
    if !((a < b) as i32 < c) {
        std::process::exit(2);
    }

    a = 3;

    if !(a < b && b < c) {
        std::process::exit(3);
    }
}