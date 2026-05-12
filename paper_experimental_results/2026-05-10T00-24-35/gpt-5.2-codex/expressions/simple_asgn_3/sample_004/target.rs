use std::process::exit;

fn main() {
    let mut c: u8 = b'A';

    let mut p: &mut u8 = &mut c;
    let mut cpp: &mut &mut u8 = &mut p;

    *cpp = &mut c;
    *p = 0;

    if c != b'A' {
        exit(1);
    }

    exit(0);
}