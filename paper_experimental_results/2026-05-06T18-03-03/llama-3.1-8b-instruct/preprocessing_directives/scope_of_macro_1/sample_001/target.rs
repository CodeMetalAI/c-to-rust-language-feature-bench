const TABSIZE: usize = 100;

fn main() -> i32 {
    let a: [i32; TABSIZE] = [0; TABSIZE];
    if a.len() != TABSIZE {
        return 1;
    }
    let mut tabsize = TABSIZE;
    tabsize = 7;
    if tabsize != 7 {
        return 2;
    }
    0
}