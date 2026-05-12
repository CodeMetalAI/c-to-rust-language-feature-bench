use std::mem::size_of;
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
            let b = x[i][j];
            if a != b {
                exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    let diff = (p1 as usize - p0 as usize) / size_of::<i32>();
    if diff != 5 {
        exit(2);
    }

    println!("PASS");
}