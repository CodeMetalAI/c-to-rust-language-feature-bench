fn main() {
    let c = 'A' as u8;
    let mut c_cell = std::cell::Cell::new(c);
    
    let p_addr = &c_cell as *const std::cell::Cell<u8> as usize;
    let cpp_value = p_addr;
    
    let p_ptr = cpp_value as *mut u8;
    unsafe {
        std::ptr::write(p_ptr, 0);
    }
    
    let final_c = c_cell.get();
    
    if final_c != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}