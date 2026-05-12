fn main() {
    const TABSIZE: usize = 100;
    let a: [i32; TABSIZE] = [0; TABSIZE];
    
    let mut tabsize: i32;
    
    if a.len() != 100 {
        std::process::exit(1);
    }
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}