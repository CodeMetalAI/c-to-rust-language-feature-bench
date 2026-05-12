fn main() -> i32 {
    let mut n: i32 = 4;
    let m: i32 = 3;
    let mut a: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
    let mut p_index: i32 = 0;
    p_index += 1;
    a[p_index as usize][2] = 99;
    if a[1][2] != 99 {
        return 1;
    }
    n = p_index - 0;
    if n != 1 {
        return 2;
    }
    0
}