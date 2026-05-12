fn main() {
    let a: i32;
    let b: i32;
    let q: i32;
    let r: i32;

    let a = 17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if q != 3 {
        std::process::exit(1);
    }
    if (q * b + r) != a {
        std::process::exit(2);
    }

    let a = -17;
    let b = 5;
    let q = a / b;
    let r = a % b;
    if q != -3 {
        std::process::exit(3);
    }
    if (q * b + r) != a {
        std::process::exit(4);
    }

    let a = 17;
    let b = -5;
    let q = a / b;
    let r = a % b;
    if q != -3 {
        std::process::exit(5);
    }
    if (q * b + r) != a {
        std::process::exit(6);
    }

    let a = -17;
    let b = -5;
    let q = a / b;
    let r = a % b;
    if q != 3 {
        std::process::exit(7);
    }
    if (q * b + r) != a {
        std::process::exit(8);
    }

    std::process::exit(0);
}