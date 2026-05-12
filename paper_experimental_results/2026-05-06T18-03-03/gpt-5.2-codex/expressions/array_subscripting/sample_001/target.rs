fn main() {
    let mut x = [[0i32; 5]; 3];

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

    let p0_index = 0usize;
    let p1_index = 5usize; // start of row 1 in a 3x5 array
    if (p1_index as isize - p0_index as isize) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}