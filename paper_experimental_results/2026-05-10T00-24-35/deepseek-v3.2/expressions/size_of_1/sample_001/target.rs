use std::alloc::{alloc, dealloc, Layout};

fn main() {
    // Allocate memory for a double
    let layout = Layout::new::<f64>();
    let ptr = unsafe { alloc(layout) as *mut f64 };
    
    if ptr.is_null() {
        std::process::exit(1);
    }
    
    // Write value
    unsafe { *ptr = 3.14 };
    
    // Check value
    if unsafe { *ptr } != 3.14 {
        unsafe { dealloc(ptr as *mut u8, layout) };
        std::process::exit(2);
    }
    
    // Clean up
    unsafe { dealloc(ptr as *mut u8, layout) };
    std::process::exit(0);
}