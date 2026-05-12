use std::process::exit;

const TABSIZE: usize = 100;
static A: [i32; TABSIZE] = [0; TABSIZE];

fn main() {
    if A.len() != 100 {
        exit(1);
    }
    let mut TABSIZE: i32 = 0;
    TABSIZE = 7;
    if TABSIZE != 7 {
        exit(2);
    }
    exit(0);
}