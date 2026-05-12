fn main() {
    let mut c = 'A';
    let mut p = &mut c;
    let mut cpp = &mut p;
    *cpp = &mut c;
    **cpp = '\0';
    if c != 'A' {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}