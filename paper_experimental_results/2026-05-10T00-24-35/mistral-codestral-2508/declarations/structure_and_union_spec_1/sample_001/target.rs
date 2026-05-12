fn main() {
    #[repr(C)]
    union U {
        i: (i32, i32),
        w: (i64, i64),
    }

    #[repr(C)]
    struct V {
        u: U,
        m: i32,
    }

    let mut v1 = V {
        u: U { i: (0, 0) },
        m: 0,
    };

    v1.u.i.0 = 2;
    v1.u.w.0 = 5;

    if v1.u.i.0 != 2 {
        std::process::exit(1);
    }

    if v1.u.w.0 != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}