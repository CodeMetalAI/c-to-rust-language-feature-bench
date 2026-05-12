fn main() {
    let p: *mut &u8;
    let c: u8 = b'A';

    unsafe {
        p = &(&c as *const u8) as *const _ as *mut &u8;
        *p = &0u8;
    }

    if c != b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}