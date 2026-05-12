fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let c = b'A';

    let cpp = p as *const *const u8;
    unsafe {
        *cpp = &c as *const u8;
        *p = 0;
    }

    if *c as u8!= b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}