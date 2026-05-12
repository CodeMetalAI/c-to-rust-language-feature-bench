fn main() -> std::process::ExitCode {
    let x = 7;
    let p = &x;

    if std::ptr::addr_of!(x) != p {
        return std::process::ExitCode::from(1);
    }

    if &(*p) != p {
        return std::process::ExitCode::from(2);
    }

    let a = [10, 20, 30];

    if std::ptr::addr_of!(a[0]) != a.as_ptr().wrapping_offset(0) {
        return std::process::ExitCode::from(3);
    }
    if std::ptr::addr_of!(a[1]) != a.as_ptr().wrapping_offset(1) {
        return std::process::ExitCode::from(4);
    }
    if std::ptr::addr_of!(a[2]) != a.as_ptr().wrapping_offset(2) {
        return std::process::ExitCode::from(5);
    }

    if unsafe { *a.as_ptr().wrapping_offset(1) } != 20 {
        return std::process::ExitCode::from(6);
    }

    std::process::ExitCode::from(0)
}