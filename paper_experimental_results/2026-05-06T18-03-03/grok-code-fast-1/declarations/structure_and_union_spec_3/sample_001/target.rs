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
    let num_a = 4;
    let total = (mem::size_of::<S>() / mem::size_of::<i32>()) + num_a;
    let mut data: Vec<i32> = vec![0; total];
    data[0] = 7;
    data[1] = 11;
    data[1 + 3] = 22;
    if data[0] != 7 || data[1] != 11 || data[1 + 3] != 22 {
        return 4;
    }
    0
}