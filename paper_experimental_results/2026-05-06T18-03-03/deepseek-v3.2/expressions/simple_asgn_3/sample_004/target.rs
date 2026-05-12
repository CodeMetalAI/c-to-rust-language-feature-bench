fn main() {
    let c = 'A';
    let c_ptr = &c as *const char;
    let mut p: *mut char = std::ptr::null_mut();
    let cpp: *const *const char = &p as *const *mut char as *const *const char;
    
    unsafe {
        *(cpp as *mut *const char) = c_ptr;
        if !p.is_null() {
            *p = '\0';
        }
    }
    
    if c != 'A' {
        std::process::exit(1);
    }
    std::process::exit(0);
}