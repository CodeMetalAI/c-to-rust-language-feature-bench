fn main() {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a;

    p = &mut a[1..];
    p += 1;

    p[0][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if (p as *const _ as usize - a as *const _ as usize) != 3 {
        return 2;
    }

    0
}