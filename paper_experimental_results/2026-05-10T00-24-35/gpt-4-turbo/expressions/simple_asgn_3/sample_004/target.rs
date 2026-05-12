fn main() {
    let mut p: *const u8 = std::ptr::null();
    let c = b'A';
    let cpp = &mut p;
    
    unsafe {
        *cpp = &c;
        std::ptr::write(cpp as *mut *const u8 as *mut u8, 0);
    }

    if c != b'A' {
        std::process::exit(1);
    }
}