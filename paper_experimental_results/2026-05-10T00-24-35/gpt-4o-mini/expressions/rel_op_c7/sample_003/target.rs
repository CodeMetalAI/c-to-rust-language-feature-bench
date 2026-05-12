fn main() -> std::process::ExitCode {
    let x = 42;
    let p = &x;

    if p as *const _ != &x as *const _ {
        return std::process::ExitCode::from(1);
    }

    let p_end1 = unsafe { (p as *const _).add(1) };
    let p_end2 = unsafe { (&x as *const _).add(1) };

    if p_end1 != p_end2 {
        return std::process::ExitCode::from(2);
    }

    if p_end1 == p as *const _ {
        return std::process::ExitCode::from(3);
    }

    std::process::ExitCode::from(0)
}