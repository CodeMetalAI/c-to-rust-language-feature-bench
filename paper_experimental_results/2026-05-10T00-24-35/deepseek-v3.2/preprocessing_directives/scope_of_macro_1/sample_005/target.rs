const TABSIZE: usize = 100;

static mut A: [i32; TABSIZE] = [0; TABSIZE];

fn main() {
    let tabsize: i32;
    
    // Safe because we're the only thread and we access it carefully
    let a_size = unsafe { A.len() };
    if a_size != 100 {
        std::process::exit(1);
    }
    
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}