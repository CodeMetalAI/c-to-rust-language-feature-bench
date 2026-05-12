fn main() {
    let mut a = [[0; 3]; 4];

    let p = a.as_mut_ptr() as *mut [i32; 3];

    let p = p.offset(1);
    (*p)[2] = 99;

    if a[1][2]!= 99 {
        std::process::exit(1);
    }

    if (p as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>()!= 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}