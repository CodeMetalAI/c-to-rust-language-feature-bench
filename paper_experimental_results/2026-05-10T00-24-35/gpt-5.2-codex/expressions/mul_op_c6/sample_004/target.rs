use std::process;

fn main() {
    let mut a: i32;
    let mut b: i32;
    let mut q: i32;
    let mut r: i32;

    a = 17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != 3 {
        process::exit(1);
    }
    if q * b + r != a {
        process::exit(2);
    }

    a = -17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != -3 {
        process::exit(3);
    }
    if q * b + r != a {
        process::exit(4);
    }

    a = 17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != -3 {
        process::exit(5);
    }
    if q * b + r != a {
        process::exit(6);
    }

    a = -17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != 3 {
        process::exit(7);
    }
    if q * b + r != a {
        process::exit(8);
    }

    process::exit(0);
}