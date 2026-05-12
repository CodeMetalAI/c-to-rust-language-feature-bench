fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * (i as i32) + (j as i32);
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

    let index0 = 0 * 5 + 0;
    let index1 = 1 * 5 + 0;
    let diff = (index1 as isize) - (index0 as isize);
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}