use std::process::exit;

fn main() {
    let mut x = [[0i32; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (i as i32) + (j as i32);
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                exit(1);
            }
        }
    }

    let p0_idx = 0usize;
    let p1_idx = 1usize * 5usize;
    if p1_idx.wrapping_sub(p0_idx) != 5 {
        exit(2);
    }

    println!("PASS");
}