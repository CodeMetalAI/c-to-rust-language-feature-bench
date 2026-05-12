fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let mut p_row = 0;
    p_row += 1;
    a[p_row][2] = 99;
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    let diff = p_row - 0;
    if diff != 1 {
        std::process::exit(2);
    }
    std::process::exit(0);
}