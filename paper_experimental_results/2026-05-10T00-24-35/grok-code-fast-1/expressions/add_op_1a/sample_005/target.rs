fn main() {
    let mut n: i32 = 4;
    let m: i32 = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p_idx = 0;

    p_idx += 1;
    a[p_idx][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    n = p_idx as i32;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}