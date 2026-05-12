fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C: !(a < b < c) evaluates as !((a < b) < c)
    // First check: a < b -> 1 < 2 -> true (1), then 1 < c -> 1 < 2 -> true (1)
    // So !(true) -> false, condition not met
    if !((a < b) < c) {
        std::process::exit(1);
    }

    // Explicit check: (a < b) < c -> (1 < 2) -> true (1), then 1 < 2 -> true
    // So !(true) -> false, condition not met
    if !((a < b) < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    // Now: a < b -> 3 < 2 -> false (0), then 0 < c -> 0 < 2 -> true (1)
    // So !(true) -> false, condition not met
    if !((a < b) < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}