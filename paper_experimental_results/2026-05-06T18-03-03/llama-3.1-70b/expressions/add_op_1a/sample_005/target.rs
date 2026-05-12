fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let p: *mut [i32; 3] = a.as_mut_ptr();

    unsafe {
        *p.offset(1).as_mut_ptr().offset(2) = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    let n = (p.offset(1) as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>();
    if n!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}