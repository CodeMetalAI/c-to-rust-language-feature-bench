fn main() {
    const ARRAY_SIZE: usize = 100;
    let a: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    if a.len() != 100 {
        std::process::exit(1);
    }
    let mut TABSIZE: i32 = 0;
    TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}