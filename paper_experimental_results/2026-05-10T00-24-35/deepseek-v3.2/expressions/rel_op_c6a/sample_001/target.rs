fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C: a < b < c is (a < b) < c, where (a < b) evaluates to 1 or 0
    // So a < b < c becomes (a < b) < c
    if !((a < b) < c) {
        std::process::exit(1);
    }

    // Same as above, explicit grouping
    if !(((a < b) < c)) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    // Now a < b is false (0), so 0 < c is true
    if !((a < b) < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}