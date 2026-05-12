fn main() {
    let c: char = 'A';
    let mut c_byte: u8 = c as u8;
    let p: *mut u8 = &mut c_byte;
    let cpp: *const *const u8 = &p as *const *mut u8 as *const *const u8;
    
    unsafe {
        let cpp_deref: *const u8 = *cpp;
        let c_ptr: *const u8 = &c_byte;
        
        if cpp_deref == p {
            *p = 0;
        }
    }
    
    if c_byte != 'A' as u8 {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}