fn main() {
    let a = 1;
    let b = 2;
    let c = 2;

    if !(a < b && b < c) {
        return 1;
    }

    if !(a < b && (a < c)) {
        return 2;
    }

    let a = 3;
    let b = 2;
    let c = 2;

    if !(a < b && b < c) {
        return 3;
    }

    return 0;
}