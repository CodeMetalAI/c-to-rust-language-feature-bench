fn alloc(n: usize) -> Option<*mut u8> {
    if n == 0 {
        return None;
    }
    // SAFETY: We ensure alignment and size are handled by the caller
    let ptr = unsafe { libc::malloc(n) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr as *mut u8)
    }
}

fn main() {
    let ptr = alloc(std::mem::size_of::<f64>());
    let dp = match ptr {
        Some(p) => p as *mut f64,
        None => return,
    };
    
    // SAFETY: We just allocated enough space for one f64 and checked it's not null
    unsafe {
        *dp = 3.14;
    }
    
    // SAFETY: We know dp is valid and properly aligned
    let value = unsafe { *dp };
    if value != 3.14 {
        // SAFETY: dp was allocated with malloc
        unsafe {
            libc::free(dp as *mut libc::c_void);
        }
        return;
    }
    
    // SAFETY: dp was allocated with malloc
    unsafe {
        libc::free(dp as *mut libc::c_void);
    }
}