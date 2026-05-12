fn main() {
    let mut x = [[0i32; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (1 * i + j) as i32;
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

    let p0 = (0isize, 0isize);
    let p1 = (1isize, 0isize);
    let diff = (p1.0 * 5 + p1.1) - (p0.0 * 5 + p0.1);
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}