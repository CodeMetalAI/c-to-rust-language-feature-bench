fn main() {
    let mut a = [[0; 3]; 4];
    
    let p = a.as_mut_ptr();
    let mut p = unsafe { p.add(1) };
    
    unsafe {
        (*p)[2] = 99;
    }
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    let p_addr = p as usize;
    let a_addr = a.as_ptr() as usize;
    let diff = (p_addr - a_addr) / std::mem::size_of::<[i32; 3]>();
    
    if diff != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}