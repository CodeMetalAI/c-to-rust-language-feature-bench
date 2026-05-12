fn main() {
    let mut a = [[0; 3]; 4];
    
    let mut p: &mut [i32; 3] = &mut a[0];
    p = &mut a[1];
    
    p[2] = 99;
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    let p_index = 1;
    let a_start = a.as_ptr() as usize;
    let p_ptr = p.as_ptr() as usize;
    let element_size = std::mem::size_of::<i32>() * 3;
    
    if (p_ptr - a_start) / element_size != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}