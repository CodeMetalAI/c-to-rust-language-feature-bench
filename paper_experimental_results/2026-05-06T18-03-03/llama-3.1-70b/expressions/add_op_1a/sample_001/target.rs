fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; m]; n];

    let p = &mut a[0] as *mut _ as *mut [i32; m];

    unsafe {
        *p.offset(1).as_mut_ptr().add(2) = 99;
    }

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    let n = (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; m]>();
    if n!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}