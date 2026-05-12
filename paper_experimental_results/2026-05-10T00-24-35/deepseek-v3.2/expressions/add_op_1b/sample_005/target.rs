fn main() {
    let mut a = [[0; 3]; 4];
    
    let mut p = a.as_mut_ptr();
    
    // p += 1 (pointer arithmetic)
    p = unsafe { p.add(1) };
    
    // (*p)[2] = 99
    unsafe {
        (*p)[2] = 99;
    }
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    // (p - a) != 1
    let p_index = unsafe { p.offset_from(a.as_ptr()) };
    if p_index != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}