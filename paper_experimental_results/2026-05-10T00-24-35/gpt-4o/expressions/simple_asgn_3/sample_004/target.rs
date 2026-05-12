fn main() {
    let p: *mut u8;
    let c: u8 = b'A';

    p = &c as *const u8 as *mut u8;
    unsafe {
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}