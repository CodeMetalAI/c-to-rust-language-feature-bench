fn main() {
    let mut a: i32;
    let b: i32;
    let q: i32;
    let r: i32;

    a = 17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != 3 {
        std::process::exit(1);
    }
    if (q * b + r) != a {
        std::process::exit(2);
    }

    a = -17;
    q = a / b;
    r = a % b;
    if q != -3 {
        std::process::exit(3);
    }
    if (q * b + r) != a {
        std::process::exit(4);
    }

    a = 17;
    let b_neg = -5;
    q = a / b_neg;
    r = a % b_neg;
    if q != -3 {
        std::process::exit(5);
    }
    if (q * b_neg + r) != a {
        std::process::exit(6);
    }

    a = -17;
    q = a / b_neg;
    r = a % b_neg;
    if q != 3 {
        std::process::exit(7);
    }
    if (q * b_neg + r) != a {
        std::process::exit(8);
    }

    std::process::exit(0);
}