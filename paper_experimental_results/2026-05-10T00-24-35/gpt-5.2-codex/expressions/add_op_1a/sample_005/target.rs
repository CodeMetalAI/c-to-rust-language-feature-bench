fn main() {
    let mut n: i32 = 4;
    let m: i32 = 3;

    let mut a = [[0i32; 3]; 4];

    let mut p_index: usize = 0;

    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    n = p_index as i32;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}