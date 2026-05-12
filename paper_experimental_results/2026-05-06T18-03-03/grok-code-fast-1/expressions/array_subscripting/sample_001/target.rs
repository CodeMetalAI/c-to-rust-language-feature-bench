use std::mem;
use std::process;

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
                process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    let diff = (p1 as usize - p0 as usize) / mem::size_of::<i32>();
    if diff as isize != 5 {
        process::exit(2);
    }

    println!("PASS");
    process::exit(0);
}