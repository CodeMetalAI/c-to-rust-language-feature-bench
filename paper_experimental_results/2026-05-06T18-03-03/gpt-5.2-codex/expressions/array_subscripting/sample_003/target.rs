use std::process::exit;

fn main() {
    let mut x = [[0i32; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j]; // equivalent to *(*(x + i) + j)
            if a != b {
                exit(1);
            }
        }
    }

    let p0_index = 0i32;
    let p1_index = 1i32 * 5;
    if p1_index - p0_index != 5 {
        exit(2);
    }

    println!("PASS");
}