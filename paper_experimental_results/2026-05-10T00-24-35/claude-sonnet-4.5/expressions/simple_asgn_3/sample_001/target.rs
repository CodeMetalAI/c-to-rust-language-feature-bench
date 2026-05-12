fn main() {
    let c: u8 = b'A';
    let p: &mut u8 = &mut c.clone();
    *p = 0;

    if c != b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}