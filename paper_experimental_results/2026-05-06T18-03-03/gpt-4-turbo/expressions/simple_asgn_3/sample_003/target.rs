fn main() {
    let mut p: *const u8;
    let c: u8 = b'A';

    p = &c;

    if c != b'A' {
        std::process::exit(1);
    }
}