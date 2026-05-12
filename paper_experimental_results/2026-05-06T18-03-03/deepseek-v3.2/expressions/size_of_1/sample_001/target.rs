use std::alloc::{alloc, dealloc, Layout};

fn alloc_double() -> *mut f64 {
    let layout = Layout::new::<f64>();
    unsafe { alloc(layout) as *mut f64 }
}

fn main() {
    let dp = alloc_double();
    
    if dp.is_null() {
        std::process::exit(1);
    }
    
    unsafe {
        *dp = 3.14;
    }
    
    let value = unsafe { *dp };
    if value != 3.14 {
        let layout = Layout::new::<f64>();
        unsafe { dealloc(dp as *mut u8, layout) };
        std::process::exit(2);
    }
    
    let layout = Layout::new::<f64>();
    unsafe { dealloc(dp as *mut u8, layout) };
    std::process::exit(0);
}