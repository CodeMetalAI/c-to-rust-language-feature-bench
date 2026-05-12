fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[i32; m]; n] = [[0; m]; n];

    let p = &mut a;

    p.iter_mut().nth(1).map(|x| *x[2] = 99);

    if a[1][2]!= 99 {
        return;
    }

    let n = (p as *mut _ as usize - a as *const _ as usize) / m;
    if n!= 1 {
        return;
    }
}