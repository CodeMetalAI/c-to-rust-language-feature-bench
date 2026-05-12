fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    let c = b'A';

    unsafe {
        *cpp = &c as *const u8;
        *p = 0;
    }

    if c!= b'A' {
        std::process::exit(1);
    }
}