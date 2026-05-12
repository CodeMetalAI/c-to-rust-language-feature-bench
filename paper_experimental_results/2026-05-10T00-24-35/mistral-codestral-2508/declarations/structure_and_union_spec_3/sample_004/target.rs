use std::mem;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let s_size = mem::size_of::<S>();
    let i_offset = mem::offset_of!(S, i);
    let a_offset = mem::offset_of!(S, a);

    if i_offset != 0 {
        std::process::exit(1);
    }

    if a_offset != s_size {
        std::process::exit(2);
    }

    let p = Box::new(S {
        i: 7,
        a: [0; 4],
    });

    if p.i != 7 || p.a[0] != 0 || p.a[3] != 0 {
        std::process::exit(4);
    }

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
}