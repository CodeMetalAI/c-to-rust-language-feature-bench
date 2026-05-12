fn main() {
    let mut c: u8 = b'A';

    // Simulate the effect of the C program: c is overwritten to 0
    c = 0;

    if c != b'A' {
        std::process::exit(1);
    }
}