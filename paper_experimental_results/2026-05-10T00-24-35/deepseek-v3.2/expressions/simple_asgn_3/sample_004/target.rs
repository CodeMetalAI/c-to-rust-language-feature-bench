use std::cell::UnsafeCell;
use std::ptr;

fn main() {
    // Use UnsafeCell to allow interior mutability of a shared reference
    let c = UnsafeCell::new(b'A');
    let mut p: *const u8 = ptr::null();
    
    // Create a mutable raw pointer to p
    let cpp: *mut *const u8 = &mut p as *mut *const u8;
    
    // Assign address of c to p through cpp
    unsafe {
        *cpp = c.get();
    }
    
    // Dereference p and write 0 to it
    unsafe {
        *(p as *mut u8) = 0;
    }
    
    // Check if c was modified
    if unsafe { *c.get() } != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}