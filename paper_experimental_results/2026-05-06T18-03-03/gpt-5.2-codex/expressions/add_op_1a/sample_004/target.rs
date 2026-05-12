use std::process::exit;

fn main() {
    let mut n: i32 = 4;
    let m: usize = 3;
    let mut a = [[0i32; 3]; 4];

    let mut p: usize = 0;
    p += 1;
    a[p][2] = 99;

    if a[1][2] != 99 {
        exit(1);
    }

    n = (p as i32) - 0;
    if n != 1 {
        exit(2);
    }

    exit(0);
}