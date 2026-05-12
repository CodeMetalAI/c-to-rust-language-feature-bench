fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p = &mut a[..] as *mut [i32; 3];

    unsafe {
        p = p.add(1);
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = unsafe { p.offset_from(&a[0]) } as i32;
    if n != 1 {
        std::process::exit(2);
    }
}