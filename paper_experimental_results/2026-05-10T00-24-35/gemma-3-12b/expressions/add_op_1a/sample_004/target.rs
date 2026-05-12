fn main() {
    let n = 4;
    let m = 3;
    let mut a = vec![vec![0; m]; n];

    let mut p = a.as_mut_slice();

    unsafe {
        p = &mut p[1];
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        return 1;
    }

    let n = {
        let ptr_diff = &a[1] as *const _ as usize - &a[0] as *const _ as usize;
        ptr_diff / m
    };

    if n != 1 {
        return 2;
    }

    0
}