fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let p = &mut a[0];

    p = &mut a[1];
    p[2] = 99;

    if a[1][2]!= 99 {
        return 1;
    }

    if a.as_ptr() as usize!= p.as_ptr() as usize {
        return 2;
    }

    return 0;
}