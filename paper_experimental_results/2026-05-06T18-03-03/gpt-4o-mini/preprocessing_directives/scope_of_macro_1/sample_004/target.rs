const TABSIZE: usize = 100;

fn main() -> i32 {
    let a: [i32; TABSIZE] = [0; TABSIZE];

    if a.len() != 100 {
        return 1;
    }

    let mut tabsize = 7;

    if tabsize != 7 {
        return 2;
    }

    0
}