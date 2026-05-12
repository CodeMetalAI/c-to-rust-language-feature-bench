fn alloc(n: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(n);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

fn main() {
    let dp = unsafe { alloc(std::mem::size_of::<f64>()) as *mut f64 };

    if dp.is_null() {
        std::process::exit(1);
    }

    unsafe {
        *dp = 3.14;
        if *dp != 3.14.0 {
            std::alloc::dealloc(dp as *mut u8, std::alloc::Layout::new::<f64>());
            std::process::exit(2);
        }
    }

    unsafe {
        std::alloc::dealloc(dp as *mut u8, std::alloc::Layout::new::<f64>());
    }
    std::process::exit(0);
}