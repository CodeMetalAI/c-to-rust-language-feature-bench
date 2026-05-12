fn main() {
    let n: usize = 4;
    let m: usize = 3;
    let mut a: [[i32; m]; n] = [[0; m]; n];

    let p: *mut [i32; m] = a.as_mut_ptr();

    unsafe {
        *p.offset(1).as_mut_ptr().add(2) = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    let n = (p as *const _ as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; m]>();
    if n!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}