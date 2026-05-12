fn main() {
    let mut c: u8 = b'A';

    {
        let p: &mut u8 = &mut c;
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}