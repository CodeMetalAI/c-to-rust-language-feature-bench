fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; m]; n];

    let mut p = a.as_mut_slice();

    unsafe {
        p = &mut *p.get_unchecked(1);
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        return 1;
    }

    let n = (p as *mut [i32; m] as *const [i32; m] as usize)
        - &a as *const [i32; m] as usize;

    if n != 1 {
        return 2;
    }

    0
}