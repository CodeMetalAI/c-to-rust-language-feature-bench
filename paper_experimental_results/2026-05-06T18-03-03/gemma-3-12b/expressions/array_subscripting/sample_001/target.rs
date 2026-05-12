fn main() {
    let mut x = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *(&x[i][j] as *const i32 as *const i32);
            if a != b {
                return 1;
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (*p1 as ptrdiff_t) - (*p0 as ptrdiff_t) != 5 {
        return 2;
    }

    println!("PASS");
    return 0;
}