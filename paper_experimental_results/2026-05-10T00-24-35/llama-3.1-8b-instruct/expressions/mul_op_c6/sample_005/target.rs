fn main() {
    let mut a = 17;
    let mut b = 5;
    let mut q = a / b;
    let mut r = a % b;
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

    return 0;
}