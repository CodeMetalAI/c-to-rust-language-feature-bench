fn main() {
    struct Inner {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    union InnerUnion {
        unnamed: Inner,
        w: W,
    }

    struct V {
        inner: InnerUnion,
        m: i32,
    }

    let mut v1 = V {
        inner: InnerUnion { unnamed: Inner { i: 0, j: 0 } },
        m: 0,
    };

    unsafe {
        v1.inner.unnamed.i = 2;
        v1.inner.w.k = 5;
    }

    let check_i;
    let check_k;
    unsafe {
        check_i = v1.inner.unnamed.i;
        check_k = v1.inner.w.k;
    }

    if check_i != 2 {
        std::process::exit(1);
    }

    if check_k != 5 {
        std::process::exit(1);
    }
}