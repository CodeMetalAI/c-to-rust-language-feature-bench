fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0_idx = 0;
    let p1_idx = 5;
    if (p1_idx - p0_idx) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}