fn main() {
    const TABSIZE: usize = 100;
    let a: [i32; TABSIZE] = [0; TABSIZE];

    if a.len() != 100 {
        std::process::exit(1);
    }

    let mut TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
}