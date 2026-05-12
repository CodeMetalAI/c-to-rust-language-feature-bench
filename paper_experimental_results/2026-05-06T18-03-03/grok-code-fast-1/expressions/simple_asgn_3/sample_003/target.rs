fn main() {
    let mut c = 'A';
    c = '\0';
    if c != 'A' {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}