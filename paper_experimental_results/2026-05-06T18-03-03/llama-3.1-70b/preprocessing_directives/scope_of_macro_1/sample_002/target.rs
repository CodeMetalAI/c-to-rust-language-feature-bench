const TABSIZE: usize = 100;

fn main() {
    let a = [0; TABSIZE];

    if a.len() != 100 {
        std::process::exit(1);
    }

    let tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}