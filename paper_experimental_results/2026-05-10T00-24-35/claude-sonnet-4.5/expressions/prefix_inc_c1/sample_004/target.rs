fn main() -> std::process::ExitCode {
    let mut x = 5;

    x += 1;
    let y = x;
    if y != 6 {
        return std::process::ExitCode::from(1);
    }
    if x != 6 {
        return std::process::ExitCode::from(2);
    }

    x = 10;
    x += 1;
    let y = x;
    let mut z = 10;
    z += 1;
    if y != 11 {
        return std::process::ExitCode::from(3);
    }
    if z != 11 {
        return std::process::ExitCode::from(4);
    }
    if z != 11 {
        return std::process::ExitCode::from(5);
    }

    std::process::ExitCode::from(0)
}