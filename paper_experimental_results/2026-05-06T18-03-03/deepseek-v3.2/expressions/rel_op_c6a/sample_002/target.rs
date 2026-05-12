fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C: !(a < b < c) means !((a < b) < c) due to left-to-right evaluation
    // First: a < b -> 1 < 2 -> true (1)
    // Then: 1 < c -> 1 < 2 -> true (1)
    // Then: !(true) -> false
    // So we don't return 1
    if !((a < b) < c) {
        std::process::exit(1);
    }

    // Explicit version of the above
    if !((a < b) < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    // Now: a < b -> 3 < 2 -> false (0)
    // Then: 0 < c -> 0 < 2 -> true (1)
    // Then: !(true) -> false
    // So we don't return 3
    if !((a < b) < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}