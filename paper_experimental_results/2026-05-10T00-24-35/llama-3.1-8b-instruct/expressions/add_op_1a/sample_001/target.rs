fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[[i32; m]; n]; 1] = [[[0; m]; n]; 1];

    let p = &mut a[0];

    p[1][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if (p as *const _ as usize - &a as *const _ as usize) as i32 != 1 {
        return 2;
    }

    return 0;
}