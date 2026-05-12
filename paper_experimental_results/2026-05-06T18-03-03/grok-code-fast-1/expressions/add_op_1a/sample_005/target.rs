fn main() -> i32 {
    let mut n: i32 = 4;
    let m: i32 = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p_index: usize = 0;

    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    n = p_index as i32 - 0;
    if n != 1 {
        return 2;
    }

    0
}