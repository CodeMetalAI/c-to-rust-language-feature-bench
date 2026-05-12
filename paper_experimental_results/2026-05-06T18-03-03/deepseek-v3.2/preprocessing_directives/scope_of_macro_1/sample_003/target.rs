const TABSIZE: usize = 100;

static mut A: [i32; TABSIZE] = [0; TABSIZE];

fn main() {
    let tabsize: i32;
    
    if (std::mem::size_of_val(&unsafe { A }) / std::mem::size_of::<i32>()) != 100 {
        std::process::exit(1);
    }
    
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}