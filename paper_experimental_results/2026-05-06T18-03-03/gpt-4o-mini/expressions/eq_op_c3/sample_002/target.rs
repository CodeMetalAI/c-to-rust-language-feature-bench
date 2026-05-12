fn main() -> i32 {
    let mut a;
    let mut b;
    let mut c;
    let mut d;

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if (a < b) == (c < d) && !(a < b) {
        return 1;
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if (a < b) == (c < d) && !(a < b) {
        return 2;
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if (a < b) == (c < d && !(a < b)) {
        return 3;
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if (a < b) == (c < d && !(a < b)) {
        return 4;
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if (a < b == c < d && !(a < b)) {
        return 5;
    }

    0
}