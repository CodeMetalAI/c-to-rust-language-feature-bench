fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let c = b'A' as *const u8;

    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    *cpp = c;
    *p = 0;

    if *c as u8!= b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}