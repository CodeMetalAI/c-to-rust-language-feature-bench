fn main() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let cpp: *mut *const u8 = &mut p;
    let c = b'A';
    unsafe {
        *cpp = &c;
        *p = 0;
    }

    if c!= b'A' {
        std::process::exit(1);
    }
}