fn main() {
    let mut p: *const u8;
    let c = b'A';

    let cpp = &mut p as *mut _ as *mut *const u8;
    unsafe {
        *cpp = &c;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}