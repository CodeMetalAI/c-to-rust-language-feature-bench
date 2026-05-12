const TABSIZE: usize = 100;

fn main() {
    let mut a: [usize; TABSIZE] = [0; TABSIZE];
    if a.len() != 100 {
        std::process::exit(1);
    }
    TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}