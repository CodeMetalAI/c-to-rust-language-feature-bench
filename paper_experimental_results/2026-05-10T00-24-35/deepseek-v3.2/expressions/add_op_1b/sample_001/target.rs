fn main() {
    let mut a = [[0; 3]; 4];
    let mut p = a.as_mut_ptr();
    
    // p += 1 (increment by 1 row of 3 ints)
    p = unsafe { p.add(1) };
    
    // (*p)[2] = 99
    unsafe {
        (*p)[2] = 99;
    }
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    // Calculate p - a (in terms of row difference)
    let p_addr = p as usize;
    let a_addr = a.as_ptr() as usize;
    let row_size = std::mem::size_of::<[i32; 3]>();
    
    if (p_addr - a_addr) / row_size != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}