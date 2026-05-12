fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a!= b {
                eprintln!("Error: a!= b");
                return;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const i32 as usize - p0 as *const i32 as usize)!= 5 {
        eprintln!("Error: p1 - p0!= 5");
        return;
    }

    println!("PASS");
}