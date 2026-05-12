fn main() {
    let mut x: [[i32; 3]; 5] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *(&x[i][0] + j);
            if a != b {
                return 1;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const i32 as usize - p0 as usize) != 5 {
        return 2;
    }

    println!("PASS");
    std::process::exit(0);
}