fn main() {
    let mut p: *const u8;
    let c: u8 = b'A';

    p = &c;
    unsafe {
        *(p as *mut u8) = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}