fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let c = 'A' as u8;

    let cpp = p as *const *const u8;
    *cpp = &c as *const u8;
    *p = 0;

    if c != 'A' as u8 {
        std::process::exit(1);
    }

    std::process::exit(0);
}