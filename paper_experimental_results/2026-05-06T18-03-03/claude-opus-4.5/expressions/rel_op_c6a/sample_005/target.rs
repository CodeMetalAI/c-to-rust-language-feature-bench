fn main() {
    let mut a = 1;
    let mut b = 2;
    let c = 2;

    // In C, `a < b < c` is evaluated as `(a < b) < c`
    // `a < b` returns 1 (true) or 0 (false), then that result is compared to c
    // So we need to convert the boolean result to an integer for the second comparison
    
    // a < b < c  =>  (a < b) < c  =>  (1 < 2) < 2  =>  1 < 2  =>  true
    // !(true) => false, so we don't return 1
    let cmp1 = if a < b { 1 } else { 0 };
    if !(cmp1 < c) {
        std::process::exit(1);
    }

    // (a < b) < c  =>  (1 < 2) < 2  =>  1 < 2  =>  true
    // !(true) => false, so we don't return 2
    let cmp2 = if a < b { 1 } else { 0 };
    if !(cmp2 < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    // c is still 2

    // a < b < c  =>  (a < b) < c  =>  (3 < 2) < 2  =>  0 < 2  =>  true
    // !(true) => false, so we don't return 3
    let cmp3 = if a < b { 1 } else { 0 };
    if !(cmp3 < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}