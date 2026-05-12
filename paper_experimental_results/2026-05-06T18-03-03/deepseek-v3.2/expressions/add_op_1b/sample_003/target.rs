fn main() {
    let mut a = [[0; 3]; 4];
    
    let p = &mut a as *mut _ as *mut [i32; 3];
    let mut p = unsafe { p.offset(1) };
    
    unsafe {
        (*p)[2] = 99;
    }
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    let p_addr = p as usize;
    let a_addr = a.as_mut_ptr() as usize;
    let element_size = std::mem::size_of::<[i32; 3]>();
    
    if (p_addr - a_addr) / element_size != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}