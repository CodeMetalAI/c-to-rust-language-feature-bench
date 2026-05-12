fn main() {
    const ARRAY_SIZE: usize = 100;
    let mut a = [0; ARRAY_SIZE];

    let mut tabsize = 0;

    if a.len() != 100 {
        std::process::exit(1);
    }

    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}