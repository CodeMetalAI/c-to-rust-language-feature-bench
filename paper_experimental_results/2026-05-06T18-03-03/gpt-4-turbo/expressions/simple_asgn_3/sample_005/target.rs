fn main() {
    let mut p: *const u8;
    let c: u8 = b'A';

    // Simulate cpp pointing to p
    let cpp: *mut *const u8 = &mut p as *mut *const u8;

    // Simulate *cpp = &c;
    unsafe {
        *cpp = &c as *const u8;
    }

    // Check if c is still 'A'
    if c != b'A' {
        std::process::exit(1);
    }
}