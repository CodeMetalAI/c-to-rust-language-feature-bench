use std::process::exit;

const TABSIZE_CONST: usize = 100;
static A: [i32; TABSIZE_CONST] = [0; TABSIZE_CONST];

fn main() {
    if A.len() != 100 {
        exit(1);
    }
    let mut tabsize: i32 = 0;
    tabsize = 7;
    if tabsize != 7 {
        exit(2);
    }
    exit(0);
}