use std::mem;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 4],
}

fn main() -> i32 {
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        return 2;
    }

    let mut p = Box::new(S { i: 0, a: [0; 4] });

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        return 4;
    }

    0
}