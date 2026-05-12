use std::mem::{offset_of, size_of};

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
    if v.try_reserve(len).is_err() {
        return None;
    }
    v.resize(len, 0.0);
    Some(v)
}

fn main() {
    let mut exit_code = 0;

    if size_of::<S>() < size_of::<SS>() {
        exit_code = 1;
    } else if offset_of!(S, d) != size_of::<S>() {
        exit_code = 1;
    } else {
        let mut s1 = match make_vec(8) {
            Some(v) => v,
            None => {
                exit_code = 1;
                Vec::new()
            }
        };
        let mut s2 = match make_vec(5) {
            Some(v) => v,
            None => {
                exit_code = 1;
                Vec::new()
            }
        };

        if exit_code == 0 {
            s1[0] = 42.0;
            s2[0] = 24.0;

            if s1[0] != 42.0 || s2[0] != 24.0 {
                exit_code = 1;
            }
        }
    }

    std::process::exit(exit_code);
}