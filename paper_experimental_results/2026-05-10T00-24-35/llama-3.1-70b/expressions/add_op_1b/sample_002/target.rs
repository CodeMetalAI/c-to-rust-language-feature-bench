fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a[0] as *mut _ as *mut [i32; 3];

    unsafe {
        *p.offset(1).as_mut().unwrap()[2] = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    let diff = unsafe { p.offset(1) as usize - p as usize } / std::mem::size_of::<[i32; 3]>();
    if diff!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}