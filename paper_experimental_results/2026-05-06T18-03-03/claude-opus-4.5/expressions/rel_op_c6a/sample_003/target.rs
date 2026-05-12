fn main() {
    let mut a = 1;
    let mut b = 2;
    let c = 2;

    // In C, `a < b < c` is evaluated as `(a < b) < c`
    // where `(a < b)` returns 0 or 1, then compared to c
    // So we need to replicate this behavior
    
    // a < b is true (1 < 2), so result is 1
    // 1 < c is true (1 < 2), so result is 1 (true)
    // !(true) is false, so we don't return 1
    let ab_result = if a < b { 1 } else { 0 };
    if !(ab_result < c) {
        std::process::exit(1);
    }

    // Same logic for (a < b) < c
    let ab_result = if a < b { 1 } else { 0 };
    if !(ab_result < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    // c remains 2

    // a < b is false (3 < 2), so result is 0
    // 0 < c is true (0 < 2), so result is 1 (true)
    // !(true) is false, so we don't return 3
    let ab_result = if a < b { 1 } else { 0 };
    if !(ab_result < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}