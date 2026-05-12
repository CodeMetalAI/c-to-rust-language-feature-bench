fn main() {
    const TABSIZE_CONST: usize = 100;
    let a: [i32; TABSIZE_CONST] = [0; TABSIZE_CONST];
    if a.len() != 100 {
        std::process::exit(1);
    }
    let TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
}