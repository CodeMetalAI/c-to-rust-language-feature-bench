use std::mem::size_of;
use std::process::exit;

struct Header {
    i: i32,
}

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    let offset_i = 0usize;
    if offset_i != 0 {
        exit(1);
    }

    let offset_a = size_of::<Header>();
    if offset_a != size_of::<Header>() {
        exit(2);
    }

    let mut p = S {
        i: 0,
        a: vec![0; 4],
    };

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        exit(4);
    }

    exit(0);
}