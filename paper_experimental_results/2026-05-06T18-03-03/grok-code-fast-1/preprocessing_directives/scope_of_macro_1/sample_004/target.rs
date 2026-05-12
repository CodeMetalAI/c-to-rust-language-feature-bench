static a: [i32; 100] = [0; 100];

fn main() {
    if a.len() != 100 {
        std::process::exit(1);
    }
    let mut TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
}