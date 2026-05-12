use std::process::exit;

fn main() {
    let mut a = [[0i32; 3]; 4];

    let mut p_index: usize = 0;
    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        exit(1);
    }

    if p_index as isize - 0isize != 1 {
        exit(2);
    }

    exit(0);
}