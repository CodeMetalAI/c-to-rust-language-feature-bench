fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p: &mut [i32; 3] = &mut a[0];

    unsafe {
        p = p.add(1);
    }

    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}