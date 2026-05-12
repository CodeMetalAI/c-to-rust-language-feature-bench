fn main() -> i32 {
    let mut n: i32 = 4;
    let _m: i32 = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let mut p: usize = 0;
    p += 1;
    a[p][2] = 99;
    if a[1][2] != 99 {
        return 1;
    }
    n = p as i32;
    if n != 1 {
        return 2;
    }
    0
}