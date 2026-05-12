const MAX: usize = 12;

fn main() -> std::process::ExitCode {
    let mut a = [0; MAX];
    a[0] = 1;
    a[1] = 3;
    a[2] = 5;
    a[3] = 7;
    a[4] = 9;
    a[MAX - 5] = 8;
    a[MAX - 4] = 6;
    a[MAX - 3] = 4;
    a[MAX - 2] = 2;
    a[MAX - 1] = 0;

    if a[0] != 1 {
        return std::process::ExitCode::from(1);
    }
    if a[1] != 3 {
        return std::process::ExitCode::from(2);
    }
    if a[2] != 5 {
        return std::process::ExitCode::from(3);
    }
    if a[3] != 7 {
        return std::process::ExitCode::from(4);
    }
    if a[4] != 9 {
        return std::process::ExitCode::from(5);
    }

    if a[MAX - 5] != 8 {
        return std::process::ExitCode::from(6);
    }
    if a[MAX - 4] != 6 {
        return std::process::ExitCode::from(7);
    }
    if a[MAX - 3] != 4 {
        return std::process::ExitCode::from(8);
    }
    if a[MAX - 2] != 2 {
        return std::process::ExitCode::from(9);
    }
    if a[MAX - 1] != 0 {
        return std::process::ExitCode::from(10);
    }

    {
        let mut i = 5;
        while i < MAX - 5 {
            if a[i] != 0 {
                return std::process::ExitCode::from(11);
            }
            i += 1;
        }
    }

    std::process::ExitCode::from(0)
}