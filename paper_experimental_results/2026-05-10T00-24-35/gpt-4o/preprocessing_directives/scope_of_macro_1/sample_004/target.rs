fn main() {
    const TABSIZE_INITIAL: usize = 100;

    let a = [0; TABSIZE_INITIAL];

    let mut tabsize: i32;

    if a.len() != 100 {
        std::process::exit(1);
    }
    
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}