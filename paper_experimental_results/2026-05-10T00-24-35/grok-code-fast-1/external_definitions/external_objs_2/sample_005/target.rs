fn main() {
    let mut i = 0;
    if i != 0 {
        std::process::exit(1);
    }
    i = 7;
    if i != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}