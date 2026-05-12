use std::mem;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() -> i32 {
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }
    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        return 2;
    }
    let mut p: Vec<i32> = vec![0; 5];
    p[0] = 7;
    p[1] = 11;
    p[4] = 22;
    if p[0] != 7 || p[1] != 11 || p[4] != 22 {
        return 4;
    }
    0
}