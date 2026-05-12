use std::mem::{size_of, offset_of};

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if size_of::<S>() < size_of::<SS>() {
        std::process::exit(1);
    }

    if offset_of!(S, d) != size_of::<S>() {
        std::process::exit(1);
    }

    let s1 = Box::new(S {
        n: 0,
        d: [0.0; 8],
    });

    let s2 = Box::new(S {
        n: 0,
        d: [0.0; 5],
    });

    unsafe {
        *s1.d.as_ptr().offset(0) = 42.0;
        *s2.d.as_ptr().offset(0) = 24.0;

        if *s1.d.as_ptr().offset(0) != 42.0 || *s2.d.as_ptr().offset(0) != 24.0 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}