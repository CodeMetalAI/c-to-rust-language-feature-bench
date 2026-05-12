fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p: usize = 0;

    p += 1;
    a[p][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n_diff = p as i32 - 0;
    if n_diff != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}