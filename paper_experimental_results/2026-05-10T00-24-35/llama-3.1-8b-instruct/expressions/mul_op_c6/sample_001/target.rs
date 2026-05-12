fn main() -> i32 {
    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if q != 3 {
        return 1;
    }
    if (q * b + r) != a {
        return 2;
    }

    a = -17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != -3 {
        return 3;
    }
    if (q * b + r) != a {
        return 4;
    }

    a = 17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != -3 {
        return 5;
    }
    if (q * b + r) != a {
        return 6;
    }

    a = -17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != 3 {
        return 7;
    }
    if (q * b + r) != a {
        return 8;
    }

    0
}