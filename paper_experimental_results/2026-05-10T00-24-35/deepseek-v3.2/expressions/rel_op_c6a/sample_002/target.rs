fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C/C++, `a < b < c` is evaluated as `(a < b) < c` where `(a < b)` yields 0 or 1
    // Rust doesn't allow chaining relational operators like this, so we need to replicate the C behavior
    if !((a < b) as i32 < c) {
        std::process::exit(1);
    }

    if !((a < b) as i32 < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !((a < b) as i32 < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}