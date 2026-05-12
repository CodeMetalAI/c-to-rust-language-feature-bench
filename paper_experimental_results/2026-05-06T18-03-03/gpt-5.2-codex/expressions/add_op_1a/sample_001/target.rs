fn main() {
    let mut n: i32 = 4;
    let m: usize = 3;
    let mut a = [[0i32; 3]; 4];

    let mut p: isize = 0;
    p += 1;
    a[p as usize][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    n = p as i32;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}