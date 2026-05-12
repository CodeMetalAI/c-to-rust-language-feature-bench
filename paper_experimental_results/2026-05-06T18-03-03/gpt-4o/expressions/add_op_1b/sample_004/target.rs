fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a as *mut [[i32; 3]; 4] as *mut [i32; 3];

    let p = unsafe { p.add(1) };
    unsafe { (*p)[2] = 99 };

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let offset = unsafe { p.offset_from(a.as_mut_ptr()) };
    if offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}