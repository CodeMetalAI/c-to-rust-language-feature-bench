use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::new::<f64>();
    let ptr = unsafe { alloc(layout) as *mut f64 };
    
    if ptr.is_null() {
        std::process::exit(1);
    }
    
    unsafe {
        *ptr = 3.14;
    }
    
    let value = unsafe { *ptr };
    if value != 3.14 {
        unsafe {
            dealloc(ptr as *mut u8, layout);
        }
        std::process::exit(2);
    }
    
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
    std::process::exit(0);
}