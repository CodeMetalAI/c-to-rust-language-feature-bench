fn main() {
    let c = 'A';
    let c_ptr = &c as *const char;
    
    let mut p: *mut char = std::ptr::null_mut();
    let cpp: *mut *const char = &mut p as *mut *mut char as *mut *const char;
    
    unsafe {
        *cpp = c_ptr;
        *p = '\0';
    }
    
    if c != 'A' {
        std::process::exit(1);
    }
}