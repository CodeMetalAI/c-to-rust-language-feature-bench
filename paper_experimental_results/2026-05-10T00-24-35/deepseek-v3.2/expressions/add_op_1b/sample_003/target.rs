fn main() {
    let mut a = [[0; 3]; 4];
    let mut p = a.as_mut_ptr();

    unsafe {
        p = p.add(1);
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p_offset = unsafe { p.offset_from(a.as_ptr()) };
    if p_offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}