fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    // In C: a < b < c evaluates (a < b) which is true (1), then 1 < c which is true (1)
    // Rust doesn't allow chaining relational operators, so we explicitly evaluate:
    if !((a < b) && (b < c)) {
        return;
    }

    // In C: (a < b) < c evaluates (a < b) which is true (1), then 1 < c which is true (1)
    if !((a < b) < c) {
        return;
    }

    a = 3;
    let b = 2;
    let c = 2;

    // In C: a < b < c evaluates (a < b) which is false (0), then 0 < c which is true (1)
    if !((a < b) && (b < c)) {
        return;
    }

    return;
}