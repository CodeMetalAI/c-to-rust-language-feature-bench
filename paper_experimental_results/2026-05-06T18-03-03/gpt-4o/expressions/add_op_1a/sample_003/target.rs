fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let p = &mut a as *mut [[i32; 3]; 4] as *mut [i32; 3];
    let p = unsafe { p.add(1) };
    unsafe { (*p)[2] = 99 };

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = unsafe { p.offset_from(a.as_mut_ptr()) };
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}