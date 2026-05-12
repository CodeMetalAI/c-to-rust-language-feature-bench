fn main() {
    let mut p: *mut u8 = Box::into_raw(Box::new(b'\0'));
    let c = b'A';
    let cpp = &mut p as *mut *const u8;

    unsafe {
        *cpp = &c as *const u8;
        *p = 0;
    }

    if *cpp.offset(0) != b'A' as u8 {
        std::process::exit(1);
    }

    std::process::exit(0);
}