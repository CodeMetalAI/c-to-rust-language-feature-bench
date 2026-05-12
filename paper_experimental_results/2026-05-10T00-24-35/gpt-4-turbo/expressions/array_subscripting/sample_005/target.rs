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
            let b = *x.get(i).and_then(|row| row.get(j)).unwrap();
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const _;
    let p1 = &x[1][0] as *const _;
    if (p1 as isize - p0 as isize) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}