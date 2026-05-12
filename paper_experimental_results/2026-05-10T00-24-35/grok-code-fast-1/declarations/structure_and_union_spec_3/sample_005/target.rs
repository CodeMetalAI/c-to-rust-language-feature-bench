use std::mem;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

struct MyPtr {
    data: Vec<i32>,
}

impl MyPtr {
    fn new() -> Self {
        Self { data: vec![0; 5] }
    }

    fn i(&mut self) -> &mut i32 {
        &mut self.data[0]
    }

    fn a(&mut self) -> &mut [i32] {
        &mut self.data[1..]
    }
}

fn main() -> i32 {
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        return 2;
    }

    let mut p = MyPtr::new();

    *p.i() = 7;
    p.a()[0] = 11;
    p.a()[3] = 22;

    if *p.i() != 7 || p.a()[0] != 11 || p.a()[3] != 22 {
        return 4;
    }

    0
}