fn main() {
    const ARRAY_SIZE: usize = 100;
    let mut a = [0; ARRAY_SIZE];

    let mut tabsize = 7;
    if a.len() != 100 {
        std::process::exit(1);
    }
    if tabsize != 7 {
        std::process::exit(2);
    }
}