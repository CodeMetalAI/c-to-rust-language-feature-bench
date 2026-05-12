fn main() {
    let mut cpp: *const i8 = std::ptr::null();
    let mut p: *mut i8 = std::ptr::null_mut();
    let c = 'A';

    cpp = &mut p as *const i8;
    unsafe {
        *cpp = &c as *const i8;
        *p = 0 as i8;
    }

    if c != 'A' {
        return 1;
    }

    0
}