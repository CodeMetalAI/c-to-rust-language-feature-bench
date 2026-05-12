fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2]!= 99 {
        return 1;
    }

    if p as *const _ as usize - &a as *const _ as usize!= 1 {
        return 2;
    }

    return 0;
}