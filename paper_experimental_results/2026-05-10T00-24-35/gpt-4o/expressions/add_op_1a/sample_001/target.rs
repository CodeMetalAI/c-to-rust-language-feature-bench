fn main() {
    let n = 4;
    let m = 3;
    let mut a = vec![[0; 3]; n];

    let p = &mut a[1] as *mut [i32; 3];

    unsafe {
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n_offset = (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>();
    if n_offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}