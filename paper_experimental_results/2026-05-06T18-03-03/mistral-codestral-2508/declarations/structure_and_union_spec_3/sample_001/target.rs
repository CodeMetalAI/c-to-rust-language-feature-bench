use std::mem;

struct S {
    i: i32,
    a: [i32; 4],
}

fn main() {
    let s_size = mem::size_of::<S>();
    let i_offset = mem::offset_of!(S, i);
    let a_offset = mem::offset_of!(S, a);

    if i_offset != 0 {
        std::process::exit(1);
    }

    if a_offset != s_size - mem::size_of::<[i32; 4]>() {
        std::process::exit(2);
    }

    let mut s = S {
        i: 7,
        a: [0; 4],
    };

    s.a[0] = 11;
    s.a[3] = 22;

    if s.i != 7 || s.a[0] != 11 || s.a[3] != 22 {
        std::process::exit(4);
    }

    std::process::exit(0);
}