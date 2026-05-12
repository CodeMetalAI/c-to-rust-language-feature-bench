use std::process::exit;

fn main() {
    let mut a = [[0i32; 3]; 4];

    let mut p: usize = 0;
    p += 1;
    a[p][2] = 99;

    if a[1][2] != 99 {
        exit(1);
    }

    if p as isize - 0 != 1 {
        exit(2);
    }

    exit(0);
}