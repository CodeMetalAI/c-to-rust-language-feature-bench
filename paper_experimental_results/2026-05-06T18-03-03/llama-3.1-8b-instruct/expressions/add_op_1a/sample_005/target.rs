fn main() {
    let mut n = 4;
    let mut m = 3;
    let mut a: Vec<Vec<i32>> = vec![vec![0; m]; n];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    let n = (p as *const Vec<Vec<i32>>).offset_from(&a as *const Vec<Vec<i32>>).wrapping_div(8) as usize;
    if n != 1 {
        return 2;
    }

    return 0;
}