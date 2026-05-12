const TABSIZE_ARRAY: usize = 100;

fn main() {
    let a: [i32; TABSIZE_ARRAY] = [0; TABSIZE_ARRAY];

    if a.len() != 100 {
        std::process::exit(1);
    }

    let mut tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}