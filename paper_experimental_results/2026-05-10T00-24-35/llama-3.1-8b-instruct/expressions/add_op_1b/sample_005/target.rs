fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = &mut a[1..];

    p[0][2] = 99;

    if a[1][2]!= 99 {
        return 1;
    }

    if p.as_ptr() as usize - a.as_ptr() as usize!= 1 {
        return 2;
    }

    0
}