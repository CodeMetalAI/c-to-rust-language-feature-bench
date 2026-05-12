fn main() -> std::process::ExitCode {
    let mut a: i32;
    let mut b: i32;
    let mut q: i32;
    let mut r: i32;

    a = 17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != 3 {
        return std::process::ExitCode::from(1);
    }
    if (q * b + r) != a {
        return std::process::ExitCode::from(2);
    }

    a = -17;
    b = 5;
    q = a / b;
    r = a % b;
    if q != -3 {
        return std::process::ExitCode::from(3);
    }
    if (q * b + r) != a {
        return std::process::ExitCode::from(4);
    }

    a = 17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != -3 {
        return std::process::ExitCode::from(5);
    }
    if (q * b + r) != a {
        return std::process::ExitCode::from(6);
    }

    a = -17;
    b = -5;
    q = a / b;
    r = a % b;
    if q != 3 {
        return std::process::ExitCode::from(7);
    }
    if (q * b + r) != a {
        return std::process::ExitCode::from(8);
    }

    std::process::ExitCode::from(0)
}