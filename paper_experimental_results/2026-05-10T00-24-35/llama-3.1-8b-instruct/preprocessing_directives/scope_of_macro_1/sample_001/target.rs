const TABSIZE: usize = 100;

#[allow(unused)]
static mut A: [i32; TABSIZE] = [0; TABSIZE];

fn main() {
    if A.len() != TABSIZE {
        return 1;
    }
    TABSIZE = 7;
    if TABSIZE != 7 {
        return 2;
    }
    0
}