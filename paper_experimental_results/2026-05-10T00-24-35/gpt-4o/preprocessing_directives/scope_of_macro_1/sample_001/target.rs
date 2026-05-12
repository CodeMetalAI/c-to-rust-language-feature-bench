fn main() {
    const TABSIZE_ARRAY: usize = 100;
    let a: [i32; TABSIZE_ARRAY] = [0; TABSIZE_ARRAY];

    let mut tabsize = 0;

    if a.len() != 100 {
        std::process::exit(1);
    }
    
    tabsize = 7;

    if tabsize != 7 {
        std::process::exit(2);
    }

    // If all checks pass, exit with code 0 (which is implicit in Rust if we reach the end of main)
}