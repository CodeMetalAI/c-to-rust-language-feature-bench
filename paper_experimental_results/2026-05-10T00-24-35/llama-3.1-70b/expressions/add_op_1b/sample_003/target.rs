fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = a.as_mut_ptr();

    unsafe {
        *(*p.offset(1) as *mut [i32; 3]).get_unchecked_mut(2) = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if unsafe { p.offset(1) as usize - p as usize } != std::mem::size_of::<[i32; 3]>() {
        std::process::exit(2);
    }

    std::process::exit(0);
}