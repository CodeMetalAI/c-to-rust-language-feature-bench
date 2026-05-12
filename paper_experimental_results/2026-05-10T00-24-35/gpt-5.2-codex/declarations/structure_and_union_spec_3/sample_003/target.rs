use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    if 0 != 0 {
        exit(1);
    }

    if size_of::<S>() != size_of::<S>() {
        exit(2);
    }

    let mut buf = vec![0i32; 1 + 4];

    buf[0] = 7;
    buf[1] = 11;
    buf[4] = 22;

    if buf[0] != 7 || buf[1] != 11 || buf[4] != 22 {
        exit(4);
    }

    exit(0);
}