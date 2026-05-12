use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::new::<f64>();
    let dp_ptr = unsafe { alloc(layout) as *mut f64 };

    if dp_ptr.is_null() {
        std::process::exit(1);
    }

    unsafe {
        *dp_ptr = 3.14;
    }

    let value = unsafe { *dp_ptr };
    if value != 3.14 {
        unsafe {
            dealloc(dp_ptr as *mut u8, layout);
        }
        std::process::exit(2);
    }

    unsafe {
        dealloc(dp_ptr as *mut u8, layout);
    }
    std::process::exit(0);
}