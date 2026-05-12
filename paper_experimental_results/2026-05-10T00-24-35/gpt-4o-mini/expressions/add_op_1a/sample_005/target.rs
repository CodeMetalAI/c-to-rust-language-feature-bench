fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // `p` points to the second row of `a`
    p[0][2] = 99; // Set a[1][2] to 99

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p.as_ptr() as usize / std::mem::size_of::<[i32; 3]>() - a.as_ptr() as usize / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}