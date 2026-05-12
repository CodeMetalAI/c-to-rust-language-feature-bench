fn main() {
    struct InnerUnion {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    union Union {
        inner: InnerUnion,
        w: W,
    }

    struct V {
        u: Union,
        m: i32,
    }

    let mut v1 = V {
        u: Union {
            inner: InnerUnion { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.u.inner.i = 2;
        v1.u.w.k = 5;
    }

    let exit_code = unsafe {
        if v1.u.inner.i != 2 {
            return 1;
        }

        if v1.u.w.k != 5 {
            return 1;
        }

        0
    };

    std::process::exit(exit_code);
}