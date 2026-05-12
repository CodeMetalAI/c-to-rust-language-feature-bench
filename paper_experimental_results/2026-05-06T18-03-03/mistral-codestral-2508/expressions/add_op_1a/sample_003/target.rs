fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[..];

    p = &mut p[1..];
    p[0][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p.as_ptr() as usize - a.as_ptr() as usize;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}