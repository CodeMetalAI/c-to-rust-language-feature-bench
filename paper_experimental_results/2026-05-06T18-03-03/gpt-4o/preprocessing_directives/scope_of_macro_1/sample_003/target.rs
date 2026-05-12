const ARRAY_SIZE: usize = 100;

fn main() -> i32 {
    let a: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];

    if a.len() != 100 {
        return 1;
    }

    let mut tabsize: i32 = 7;
    if tabsize != 7 {
        return 2;
    }

    0
}