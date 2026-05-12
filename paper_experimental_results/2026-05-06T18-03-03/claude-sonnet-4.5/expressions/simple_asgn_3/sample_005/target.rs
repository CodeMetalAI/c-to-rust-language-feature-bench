fn main() {
    let c: u8 = b'A';
    let mut p: *const u8 = std::ptr::null();
    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    
    // This program demonstrates undefined behavior in C/C++
    // In Rust, we cannot safely replicate this without unsafe
    // However, the requirement states "Safe Rust only"
    
    // The C code performs type punning and writes through a const pointer,
    // which is undefined behavior. The check `if (c != 'A')` would typically
    // pass because the UB may not actually modify c in practice.
    
    // In safe Rust, we cannot replicate the pointer aliasing and const
    // violation. The closest safe equivalent that preserves the observable
    // behavior (returning 0) is:
    
    if c != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}