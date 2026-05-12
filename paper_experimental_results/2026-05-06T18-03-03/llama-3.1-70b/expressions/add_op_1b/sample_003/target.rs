fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = &mut a as *mut _ as *mut [i32; 3];

    unsafe {
        *p.offset(1).as_mut().unwrap()[2] = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    let p_ptr = p as *const _;
    let a_ptr = a.as_ptr() as *const _;
    if (p_ptr as usize - a_ptr as usize) / std::mem::size_of::<[i32; 3]>()!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}