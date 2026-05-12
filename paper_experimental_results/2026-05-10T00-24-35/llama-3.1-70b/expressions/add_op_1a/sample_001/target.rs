fn main() {
    let mut n = 4;
    let m = 3;
    let mut a = vec![vec![0; m]; n];

    let p = a.as_mut_ptr() as *mut [m as usize];

    unsafe {
        *(p.offset(1)).offset(2) = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    n = (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; m]>();
    if n!= 1 {
        std::process::exit(2);
    }
}