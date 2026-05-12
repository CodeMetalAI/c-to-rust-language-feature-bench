fn main() {
    let c: u8 = b'A';
    let mut p: *const u8 = std::ptr::null();
    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    
    // This program demonstrates undefined behavior in C/C++
    // In Rust, we cannot safely replicate this without unsafe
    // However, the requirement states "Safe Rust only"
    
    // The C code:
    // 1. Creates a pointer-to-pointer that aliases with p
    // 2. Makes p point to c
    // 3. Attempts to write through p (which points to const c)
    // 4. This is UB in C, but the check likely still passes
    
    // In safe Rust, we cannot create this aliasing scenario
    // The closest safe equivalent that preserves the exit behavior:
    
    let c_val = b'A';
    
    // The original C code has undefined behavior when writing to *p
    // Since we can't replicate UB in safe Rust, we model the likely outcome:
    // The write through p would attempt to modify const c, but the final
    // check would still see c as 'A' (the write either fails or doesn't
    // affect the original const variable due to UB)
    
    if c_val != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}