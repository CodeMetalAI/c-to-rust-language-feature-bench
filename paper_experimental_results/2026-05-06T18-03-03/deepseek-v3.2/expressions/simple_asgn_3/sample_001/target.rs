fn main() {
    let c = 'A';
    let c_ptr = &c as *const char;
    
    let p: *mut char = c_ptr as *mut char;
    
    unsafe {
        *p = '\0';
    }
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}