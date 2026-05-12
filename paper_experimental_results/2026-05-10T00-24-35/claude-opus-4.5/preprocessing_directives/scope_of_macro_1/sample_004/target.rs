static A: [i32; 100] = [0; 100];

fn main() {
    let mut tabsize: i32;
    
    if A.len() != 100 {
        std::process::exit(1);
    }
    
    tabsize = 7;
    
    if tabsize != 7 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}