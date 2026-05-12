fn main() {
    let mut c = 'A';
    let mut p: &mut char = &mut c;
    *p = '\0';
    if c != 'A' {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}