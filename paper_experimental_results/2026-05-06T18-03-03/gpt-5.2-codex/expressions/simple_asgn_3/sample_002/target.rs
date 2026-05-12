use std::cell::Cell;
use std::process::exit;

fn main() {
    let c = Cell::new(b'A');

    let mut p: &Cell<u8> = &c;
    let cpp: &mut &Cell<u8> = &mut p;

    *cpp = &c;
    p.set(0);

    if c.get() != b'A' {
        exit(1);
    }

    exit(0);
}