fn main() {
    let mut a = [[0; 3]; 4];

    let p: *mut [i32; 3] = a.as_mut_ptr();

    let p = unsafe { p.offset(1) };
    unsafe { (*p)[2] = 99; }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (p as isize - a.as_ptr() as isize) / std::mem::size_of::<[i32; 3]>() as isize != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}