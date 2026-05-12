fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let c: u8 = b'A' as u8;

    let cpp: *mut *const u8 = p as *mut *const u8;
    *cpp = &c as *const u8;
    *p = 0;

    if c != b'A' as u8 {
        std::process::exit(1);
    }

    std::process::exit(0);
}