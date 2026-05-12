fn main() -> std::process::ExitCode {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != true {
        return std::process::ExitCode::from(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if ((a < b) == (c < d)) != true {
        return std::process::ExitCode::from(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if ((a < b) == (c < d)) != false {
        return std::process::ExitCode::from(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != false {
        return std::process::ExitCode::from(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != true {
        return std::process::ExitCode::from(5);
    }

    std::process::ExitCode::from(0)
}