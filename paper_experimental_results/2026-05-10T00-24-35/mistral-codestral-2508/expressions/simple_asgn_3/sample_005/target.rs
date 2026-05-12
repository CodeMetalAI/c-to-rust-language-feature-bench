fn main() {
    let mut p: *mut u8;
    let cpp: *mut *const u8;
    let c: u8 = b'A';

    unsafe {
        p = &mut 0u8;
        cpp = &mut p as *mut *mut u8 as *mut *const u8;
        *cpp = &c;
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}