fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let p_row = 1;
    a[p_row][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if p_row != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}