fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    // In C: a < b < c evaluates as (a < b) < c, where (a < b) is 1 (true)
    // So 1 < c (2) is true
    if !((a < b) < c) {
        return;
    }

    // Explicitly (a < b) < c, same as above
    if !((a < b) < c) {
        return;
    }

    a = 3;
    b = 2;
    c = 2;

    // Now a < b is false (0), so 0 < c (2) is true
    if !((a < b) < c) {
        return;
    }

    // Exit with code 0
}