const TABSIZE: usize = 100;

fn main() {
    let mut a: [i32; TABSIZE] = [0; TABSIZE];

    if a.len() != 100 {
        return 1;
    }

    TABSIZE = 7;
    if TABSIZE != 7 {
        return 2;
    }

    0
}