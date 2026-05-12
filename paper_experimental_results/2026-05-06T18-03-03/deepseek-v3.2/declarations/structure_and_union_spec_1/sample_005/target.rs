fn main() {
    struct Inner {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    struct V {
        inner: Inner,
        w: W,
        m: i32,
    }

    let mut v1 = V {
        inner: Inner { i: 0, j: 0 },
        w: W { k: 0, l: 0 },
        m: 0,
    };

    v1.inner.i = 2;
    v1.w.k = 5;

    if v1.inner.i != 2 {
        return;
    }

    if v1.w.k != 5 {
        return;
    }
}