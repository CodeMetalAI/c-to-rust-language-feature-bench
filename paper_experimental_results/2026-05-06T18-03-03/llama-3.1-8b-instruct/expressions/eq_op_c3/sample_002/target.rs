fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    if (a < b == c < d) != 1 {
        return 1;
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if (a < b == c < d) != 1 {
        return 2;
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if (a < b == c < d) != 0 {
        return 3;
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if (a < b == c < d) != 0 {
        return 4;
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if (a < b == c < d) != 1 {
        return 5;
    }

    return 0;
}