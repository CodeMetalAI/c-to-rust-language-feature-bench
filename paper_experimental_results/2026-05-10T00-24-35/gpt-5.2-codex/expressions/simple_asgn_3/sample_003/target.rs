fn main() {
    let mut c: u8 = b'A';

    // Simulate the effect of the C code where *p = 0 writes through to c.
    c = 0;

    if c != b'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}