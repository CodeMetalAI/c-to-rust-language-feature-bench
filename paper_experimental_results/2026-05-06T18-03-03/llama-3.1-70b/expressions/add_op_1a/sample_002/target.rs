fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let p = &mut a[0] as *mut [i32; 3];

    unsafe {
        *p.offset(1).as_mut().unwrap()[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = (p.offset(1) as usize - p as usize) / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}