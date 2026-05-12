fn main() {
    let c: char = 'A';
    let mut p: char = c;

    p = '\0';

    if c != 'A' {
        std::process::exit(1);
    }
}