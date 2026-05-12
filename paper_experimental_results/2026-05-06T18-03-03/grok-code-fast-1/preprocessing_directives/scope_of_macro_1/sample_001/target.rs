use std::process;

fn main() {
    let a: [i32; 100] = [0; 100];
    if a.len() != 100 {
        process::exit(1);
    }
    let mut TABSIZE = 0i32;
    TABSIZE = 7;
    if TABSIZE != 7 {
        process::exit(2);
    }
    process::exit(0);
}