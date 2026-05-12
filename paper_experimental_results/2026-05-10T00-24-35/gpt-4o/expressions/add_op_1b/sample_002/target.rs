fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a as *mut _ as *mut [i32; 3];
    let p = unsafe { p.offset(1) };
    unsafe {
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let offset = unsafe { (p as *const _ as usize) - (a.as_ptr() as usize) } / std::mem::size_of::<[i32; 3]>();
    
    if offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}