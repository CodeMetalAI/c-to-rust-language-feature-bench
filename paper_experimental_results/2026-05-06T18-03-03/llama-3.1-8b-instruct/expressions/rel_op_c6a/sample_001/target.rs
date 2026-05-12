fn main() {
    let a = 1;
    let b = 2;
    let c = 2;

    if !(a < b && b < c) {
        return 1;
    }

    if !(a < b && (b < c)) {
        return 2;
    }

    a = 3;
    b = 2;
    c = 2;

    if !(a < b && b < c) {
        return 3;
    }

    0
}