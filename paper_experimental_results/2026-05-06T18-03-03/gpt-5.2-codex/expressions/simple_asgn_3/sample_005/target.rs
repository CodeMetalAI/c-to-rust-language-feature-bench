fn main() {
    let mut c: u8 = b'A';
    {
        let p = &mut c;
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}