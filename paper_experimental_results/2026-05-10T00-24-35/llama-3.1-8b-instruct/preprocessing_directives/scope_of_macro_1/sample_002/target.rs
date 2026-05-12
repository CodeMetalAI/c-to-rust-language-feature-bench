const TABSIZE: i32 = 100;

fn main() {
    let a: [i32; TABSIZE as usize] = [0; TABSIZE as usize];
    if a.len() != TABSIZE {
        return 1;
    }
    TABSIZE = 7;
    if TABSIZE != 7 {
        return 2;
    }
    return 0;
}