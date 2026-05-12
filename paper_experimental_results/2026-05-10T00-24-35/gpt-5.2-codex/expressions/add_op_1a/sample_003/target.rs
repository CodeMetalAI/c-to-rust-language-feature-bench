use std::process::exit;

fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0i32; 3]; 4];

    let mut p_index = 0usize;
    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        exit(1);
    }

    let n_val = p_index as i32;
    if n_val != 1 {
        exit(2);
    }

    exit(0);
}