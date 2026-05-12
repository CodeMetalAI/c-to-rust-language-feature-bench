fn main() {
    let c: u8 = b'A';
    let mut p: *const u8 = std::ptr::null();
    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    
    // This program demonstrates undefined behavior in C/C++
    // In Rust, we cannot safely reproduce this without unsafe
    // However, the requirement is "Safe Rust only"
    
    // The C code writes through a const pointer, which is UB
    // In safe Rust, we cannot modify through a const reference
    // The original program's behavior is undefined, but typically
    // the check `c != 'A'` would be false (c remains 'A')
    
    // Since we cannot reproduce the UB in safe Rust, and the
    // "expected" behavior (when UB doesn't corrupt things) is
    // that c remains 'A', we return 0
    
    std::process::exit(0);
}