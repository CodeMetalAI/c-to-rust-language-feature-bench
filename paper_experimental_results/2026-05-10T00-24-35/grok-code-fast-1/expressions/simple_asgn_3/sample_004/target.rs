fn main() {
    let mut c: char = 'A';
    let p: &mut char = &mut c;
    *p = '\0';
    if c != 'A' {
        std::process::exit(1);
    }
}