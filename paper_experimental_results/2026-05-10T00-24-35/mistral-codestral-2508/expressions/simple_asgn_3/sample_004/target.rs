fn main() {
    let c: char = 'A';
    let mut p: *mut u8 = std::ptr::null_mut();
    let cpp: *mut *const u8 = &mut p as *mut *mut u8 as *mut *const u8;

    unsafe {
        *cpp = &c as *const char as *const u8;
        *p = 0;
    }

    if c != 'A' {
        std::process::exit(1);
    }
}