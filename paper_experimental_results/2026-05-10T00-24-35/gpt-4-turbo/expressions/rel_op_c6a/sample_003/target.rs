fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    // In Rust, relational chaining (a < b < c) as in Python is not possible.
    // Each comparison must be handled separately and linked with logical operators if needed.
    if !(a < b && b < c) {
        std::process::exit(1);
    }

    // The expression `(a < b) < c` in C++ is true if `a < b` is true, as `true` can be promoted to `1` which is `< c (2)`.
    // In Rust, comparison operators return a boolean, and you cannot compare a boolean with an integer directly.
    if !((a < b) as i32) < c {
        std::process::exit(2);
    }

    a = 3;

    if !(a < b && b < c) {
        std::process::exit(3);
    }
}