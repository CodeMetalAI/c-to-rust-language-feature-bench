fn main() {
    const TABSIZE: i32 = 100;
    let a: [i32; 100] = [0; 100];

    let tabsize_size = a.len() as i32;
    if tabsize_size != 100 {
        std::process::exit(1);
    }

    let mut tabsize: i32 = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}