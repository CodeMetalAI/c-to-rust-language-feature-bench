fn main() {
    let mut a = 1;
    let mut b = 2;
    let c = 2;

    // In C, `a < b < c` is evaluated as `(a < b) < c`
    // where `(a < b)` returns 0 or 1 (as int), then compared to c
    // So we need to replicate this behavior
    
    // a < b is true (1 < 2), so result is 1 (as int)
    // then 1 < c (1 < 2) is true
    let cmp1 = if a < b { 1 } else { 0 };
    if !(cmp1 < c) {
        std::process::exit(1);
    }

    // Same logic explicitly written
    let cmp2 = if a < b { 1 } else { 0 };
    if !(cmp2 < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    // c stays 2

    // a < b is false (3 < 2 is false), so result is 0
    // then 0 < c (0 < 2) is true
    let cmp3 = if a < b { 1 } else { 0 };
    if !(cmp3 < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}