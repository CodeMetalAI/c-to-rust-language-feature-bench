fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let c = 'A' as u8;

    let cpp = p as *mut *const u8;
    *cpp = &c as *const u8;
    unsafe {
        *p = 0;
    }

    if *cpp as u8!= c {
        std::process::exit(1);
    }

    std::process::exit(0);
}