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
            let b = x[i][j]; // This is the same as a, since Rust guarantees safe array access
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const _ as usize;
    let p1 = &x[1][0] as *const _ as usize;
    if (p1 - p0) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}