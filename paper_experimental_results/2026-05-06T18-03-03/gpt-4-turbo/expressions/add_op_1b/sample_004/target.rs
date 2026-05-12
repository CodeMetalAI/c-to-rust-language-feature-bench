fn main() {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..];

    p[0][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (&p[0] as *const _ as usize - &a[0] as *const _ as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}