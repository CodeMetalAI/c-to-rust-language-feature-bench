fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C: a < b < c is evaluated as (a < b) < c, where (a < b) is 0 or 1
    // So we need to replicate that behavior exactly
    if !((a < b) as i32 < c) {
        std::process::exit(1);
    }

    if !((a < b) < c) {
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