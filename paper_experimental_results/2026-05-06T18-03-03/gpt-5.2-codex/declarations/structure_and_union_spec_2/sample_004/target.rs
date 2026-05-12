use std::mem::{size_of, offset_of};
use std::process::exit;

#[repr(C)]
struct S {
    n: i32,
    d: [f64; 0],
}

#[repr(C)]
struct SS {
    n: i32,
}

fn make_vec(len: usize) -> Option<Vec<f64>> {
    let mut v = Vec::new();
    if v.try_reserve_exact(len).is_err() {
        return None;
    }
    v.resize(len, 0.0);
    Some(v)
}

fn main() {
    let mut code = 0;

    if size_of::<S>() < size_of::<SS>() {
        code = 1;
    } else if offset_of!(S, d) != size_of::<S>() {
        code = 1;
    } else {
        let s1 = make_vec(8);
        let s2 = make_vec(5);

        if s1.is_none() || s2.is_none() {
            code = 1;
        } else {
            let mut s1 = s1.unwrap();
            let mut s2 = s2.unwrap();

            s1[0] = 42.0;
            s2[0] = 24.0;

            if s1[0] != 42.0 || s2[0] != 24.0 {
                code = 1;
            }
        }
    }

    exit(code);
}