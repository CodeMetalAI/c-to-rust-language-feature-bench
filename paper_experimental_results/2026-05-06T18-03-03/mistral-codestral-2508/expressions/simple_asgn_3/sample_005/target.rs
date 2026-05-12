fn main() {
    let mut p: *mut u8;
    let c: u8 = b'A';
    let cpp: *mut *const u8;

    unsafe {
        p = &mut c as *mut u8;
        cpp = &mut p as *mut *mut u8 as *mut *const u8;
        *cpp = &c as *const u8;
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}