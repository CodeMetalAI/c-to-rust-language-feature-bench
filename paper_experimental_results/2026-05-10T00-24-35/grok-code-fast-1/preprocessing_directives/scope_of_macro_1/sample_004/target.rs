const ARRAY_SIZE: usize = 100;

fn main() {
    let a: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    if a.len() != 100 {
        std::process::exit(1);
    }
    let mut tabsize: i32 = 0; // Equivalent to int TABSIZE; in global scope, but behavior preserved
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}