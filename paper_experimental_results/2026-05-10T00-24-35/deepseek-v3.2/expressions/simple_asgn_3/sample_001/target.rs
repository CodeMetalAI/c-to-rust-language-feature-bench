fn main() {
    let c = 'A';
    let p: *mut char = std::ptr::null_mut();
    let cpp: *mut *const char = &mut p as *mut *mut char as *mut *const char;
    unsafe {
        *cpp = &c as *const char;
        *p = '\0';
    }
    
    if c != 'A' {
        std::process::exit(1);
    }
    std::process::exit(0);
}