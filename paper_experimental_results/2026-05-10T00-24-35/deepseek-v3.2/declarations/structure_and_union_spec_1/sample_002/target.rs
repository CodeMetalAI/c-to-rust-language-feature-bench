fn main() {
    struct Inner {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    union U {
        inner: Inner,
        w: W,
    }

    struct V {
        u: U,
        m: i32,
    }

    let mut v1 = V {
        u: U {
            inner: Inner { i: 0, j: 0 },
        },
        m: 0,
    };

    // SAFETY: Only one field of the union is accessed at a time.
    unsafe {
        v1.u.inner.i = 2;
        v1.u.w.k = 5;
    }

    unsafe {
        if v1.u.inner.i != 2 {
            std::process::exit(1);
        }

        if v1.u.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}