fn main() {
    let c = 'A';
    let mut p: *const char = &c;
    let cpp: *const *const char = &p;
    
    unsafe {
        // This is inherently unsafe due to modifying a const char through a pointer.
        // In Rust, we need to use unsafe to achieve the same behavior.
        let mut_ptr = p as *mut char;
        *mut_ptr = '\0';
    }

    if c != 'A' {
        return;
    }
}