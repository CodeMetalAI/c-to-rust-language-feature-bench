fn main() {
    let mut cpp: *const &str = std::ptr::null();
    let mut p: *mut i8 = std::ptr::null_mut();
    let c = 'A';

    cpp = &p as *const &str;
    unsafe {
        *cpp = &c as *const &str;
    }
    unsafe {
        *p = 0;
    }

    if c != 'A' {
        return 1;
    }

    return 0;
}