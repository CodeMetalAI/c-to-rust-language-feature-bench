fn main() {
    let n = 4;
    let m = 3;
    let mut a = vec![vec![0; m]; n];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    let n = (p as *const _ as usize - a.as_ptr() as usize) / m;
    if n != 1 {
        return 2;
    }

    return 0;
}