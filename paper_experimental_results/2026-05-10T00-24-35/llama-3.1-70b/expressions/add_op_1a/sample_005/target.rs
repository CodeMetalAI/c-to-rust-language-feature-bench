fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let p: *mut [i32; 3] = a.as_mut_ptr();

    p = (p as *mut [i32; 3]).offset(1);
    unsafe {
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = (p as *const _ as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        std::process::exit(2);
    }
}