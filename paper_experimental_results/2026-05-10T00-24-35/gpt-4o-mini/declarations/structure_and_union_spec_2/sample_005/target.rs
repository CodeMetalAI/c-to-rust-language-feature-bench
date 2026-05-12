use std::mem;

struct S {
    n: i32,
    d: Vec<f64>,
}

struct SS {
    n: i32,
}

fn main() -> Result<(), i32> {
    if mem::size_of::<S>() < mem::size_of::<SS>() {
        return Err(1);
    }

    if mem::offset_of!(S, d) != mem::size_of::<S>() {
        return Err(1);
    }

    let s1 = vec![0.0; 8];
    let s2 = vec![0.0; 5];

    let mut s1_struct = S { n: 0, d: s1 };
    let mut s2_struct = S { n: 0, d: s2 };

    s1_struct.d[0] = 42.0;
    s2_struct.d[0] = 24.0;

    if s1_struct.d[0] != 42.0 || s2_struct.d[0] != 24.0 {
        return Err(1);
    }

    Ok(())
}