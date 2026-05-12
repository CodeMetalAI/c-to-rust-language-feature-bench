fn main() {
    let (mut a, mut b, mut q, mut r): (i32, i32, i32, i32);

    a = 17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != 3 {
        std::process::exit(1);
    }
    if q * b + r != a {
        std::process::exit(2);
    }

    a = -17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != -3 {
        std::process::exit(3);
    }
    if q * b + r != a {
        std::process::exit(4);
    }

    a = 17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != -3 {
        std::process::exit(5);
    }
    if q * b + r != a {
        std::process::exit(6);
    }

    a = -17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != 3 {
        std::process::exit(7);
    }
    if q * b + r != a {
        std::process::exit(8);
    }

    std::process::exit(0);
}