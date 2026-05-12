fn main() -> i32 {
    let n = 4;
    let m = 3;
    let mut a: Vec<Vec<i32>> = vec![vec![0; m]; n];

    let mut p_row: usize = 0;

    p_row += 1;
    a[p_row][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    let n_diff = p_row as i32;
    if n_diff != 1 {
        return 2;
    }

    0
}