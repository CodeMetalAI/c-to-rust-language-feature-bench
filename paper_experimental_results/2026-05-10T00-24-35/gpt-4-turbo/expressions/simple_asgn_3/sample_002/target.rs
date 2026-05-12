fn main() {
    let mut p: *const u8;
    let c = 'A' as u8;

    p = &c;

    if c != 'A' as u8 {
        std::process::exit(1);
    }
}