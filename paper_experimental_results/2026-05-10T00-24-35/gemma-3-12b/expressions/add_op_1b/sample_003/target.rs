fn main() {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a;

    p = &mut a[1..].as_mut_slice()[0];
    p[2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if (p as *mut [_][3] as usize - &a as *const [_][3] as usize) != 1 * 3 * 4 {
        return 2;
    }

    0
}