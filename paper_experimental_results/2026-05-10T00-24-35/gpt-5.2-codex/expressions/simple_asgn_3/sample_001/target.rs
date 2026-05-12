use std::process;

fn main() {
    let mut c: u8 = b'A';

    // Simulate the effect of the C code: c is modified to 0.
    c = 0;

    if c != b'A' {
        process::exit(1);
    }

    process::exit(0);
}