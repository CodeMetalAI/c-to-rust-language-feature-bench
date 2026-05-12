fn main() {
    let mut n: i32 = 4;
    let m: i32 = 3;
    let mut a: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];

    let mut p_row: usize = 0;

    p_row += 1;
    a[p_row][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    n = (p_row - 0) as i32;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}