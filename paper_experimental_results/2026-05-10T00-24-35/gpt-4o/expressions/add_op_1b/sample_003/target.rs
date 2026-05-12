fn main() {
    let mut a = [[0; 3]; 4];

    let p: *mut [i32; 3] = a.as_mut_ptr();

    let p = unsafe { p.add(1) };
    unsafe { (*p)[2] = 99 };

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let offset = (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>();
    if offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}