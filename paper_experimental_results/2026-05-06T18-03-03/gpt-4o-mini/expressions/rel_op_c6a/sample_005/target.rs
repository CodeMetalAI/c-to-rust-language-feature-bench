fn main() -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    if !(a < b && b < c) {
        return 1;
    }

    if !((a < b) && (b < c)) {
        return 2;
    }

    a = 3;
    b = 2;
    c = 2;

    if !(a < b && b < c) {
        return 3;
    }

    return 0;
}