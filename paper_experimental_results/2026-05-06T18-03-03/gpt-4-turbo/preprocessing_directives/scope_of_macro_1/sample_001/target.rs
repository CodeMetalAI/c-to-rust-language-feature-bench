fn main() {
    const TABSIZE: usize = 100;
    let mut a = [0; TABSIZE];

    let mut tabsize = 0;
    if a.len() != 100 {
        std::process::exit(1);
    }
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}