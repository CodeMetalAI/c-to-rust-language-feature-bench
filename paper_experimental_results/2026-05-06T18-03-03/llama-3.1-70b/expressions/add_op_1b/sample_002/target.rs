fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = &mut a as *mut _ as *mut [i32; 3];

    unsafe {
        *p.offset(1).as_mut().unwrap() = [0, 0, 99];
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        std::process::exit(2);
    }
}