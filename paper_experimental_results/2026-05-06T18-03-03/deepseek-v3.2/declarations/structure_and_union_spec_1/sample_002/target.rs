fn main() {
    #[repr(C)]
    struct Inner {
        i: i32,
        j: i32,
    }

    #[repr(C)]
    struct W {
        k: i64,
        l: i64,
    }

    #[repr(C)]
    union Union {
        inner: Inner,
        w: W,
    }

    #[repr(C)]
    struct V {
        u: Union,
        m: i32,
    }

    let mut v1 = V {
        u: Union {
            inner: Inner { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.u.inner.i = 2;
        v1.u.w.k = 5;
    }

    unsafe {
        if v1.u.inner.i != 2 {
            std::process::exit(1);
        }
    }

    unsafe {
        if v1.u.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}