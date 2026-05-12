fn main() {
    let mut p: *mut i8 = std::ptr::null_mut();
    let cpp: *mut *const i8 = &mut p as *mut *mut i8 as *mut *const i8;
    let c: i8 = b'A';

    unsafe {
        *cpp = &c as *const i8;
        *p = 0;
    }

    if unsafe { *cpp.read() }!= b'A' {
        std::process::exit(1);
    }
}