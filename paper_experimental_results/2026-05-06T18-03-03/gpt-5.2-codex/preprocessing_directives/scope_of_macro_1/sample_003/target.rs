static A: [i32; 100] = [0; 100];

fn main() {
    if A.len() != 100 {
        std::process::exit(1);
    }
    let mut tabsize = 0;
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}