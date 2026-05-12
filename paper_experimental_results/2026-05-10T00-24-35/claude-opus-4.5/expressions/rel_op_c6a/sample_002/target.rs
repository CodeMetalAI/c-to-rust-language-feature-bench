fn main() {
    let mut a = 1;
    let mut b = 2;
    let c = 2;

    // In C, `a < b < c` is evaluated as `(a < b) < c`
    // `a < b` returns 1 (true) or 0 (false), then that is compared to c
    // So we need to convert the boolean result to an integer for the second comparison
    
    // a < b < c means ((a < b) as i32) < c
    if !(((a < b) as i32) < c) {
        std::process::exit(1);
    }

    // (a < b) < c is the same as above
    if !(((a < b) as i32) < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    // c = 2; // c is already 2

    // a < b is false (3 < 2 is false), so (a < b) as i32 = 0
    // 0 < 2 is true, so the condition !(true) is false
    if !(((a < b) as i32) < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}