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
            let idx = i * 5 + j;
            let b = *x.iter().flatten().nth(idx).unwrap();
            if a != b {
                exit(1);
            }
        }
    }

    let p0_index = 0usize;
    let p1_index = 5usize;
    if (p1_index as isize - p0_index as isize) != 5 {
        exit(2);
    }

    println!("PASS");
}