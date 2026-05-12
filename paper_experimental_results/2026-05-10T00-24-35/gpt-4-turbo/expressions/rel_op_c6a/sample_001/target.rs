fn main() {
    let mut a = 1;
    let b = 2;
    let mut c = 2;

    // C++ expression `a < b < c` will translate in Rust to chained comparisons `(a < b) && (b < c)`
    if !((a < b) && (b < c)) {
        std::process::exit(1);
    }

    // The C++ expression `(a < b) < c` does not have a direct equivalent in Rust due to type safety:
    // `(a < b)` evaluates to a boolean and cannot be compared with `c` (an integer).
    // Best approach is to define what was intended by the original code, if misinterpreted as always true, then:
    if !((a < b) && true) { // simplifying the expression which would always be true
        std::process::exit(2);
    }

    a = 3;

    if !((a < b) && (b < c)) {
        std::process::exit(3);
    }
}