fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = a.as_mut_ptr() as *mut [i32; 3];

    unsafe {
        *p.offset(1).as_mut_ptr().offset(2) = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p = p as *const [i32; 3];
    let a = a.as_ptr() as *const [i32; 3];
    if (p as usize - a as usize) != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}