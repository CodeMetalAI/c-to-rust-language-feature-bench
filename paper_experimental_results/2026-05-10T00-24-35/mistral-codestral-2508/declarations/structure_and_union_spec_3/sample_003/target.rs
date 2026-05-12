use std::mem;

struct S {
    i: i32,
    a: [i32; 4],
}

fn main() {
    let s = S {
        i: 7,
        a: [11, 0, 0, 22],
    };

    if s.i != 7 || s.a[0] != 11 || s.a[3] != 22 {
        std::process::exit(4);
    }
}