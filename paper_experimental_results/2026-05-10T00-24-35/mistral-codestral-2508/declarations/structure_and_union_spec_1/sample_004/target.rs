fn main() {
    #[repr(C)]
    union U {
        s1: S1,
        s2: S2,
    }

    #[repr(C)]
    struct S1 {
        i: i32,
        j: i32,
    }

    #[repr(C)]
    struct S2 {
        k: i64,
        l: i64,
    }

    #[repr(C)]
    struct V {
        u: U,
        m: i32,
    }

    let mut v1 = V {
        u: U {
            s1: S1 { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.u.s1.i = 2;
        v1.u.s2.k = 5;
    }

    if v1.u.s1.i != 2 {
        std::process::exit(1);
    }

    if unsafe { v1.u.s2.k } != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}