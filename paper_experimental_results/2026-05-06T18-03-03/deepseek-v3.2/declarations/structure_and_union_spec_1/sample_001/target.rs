fn main() {
    struct Inner {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    union VUnion {
        inner: Inner,
        w: W,
    }

    struct V {
        union_data: VUnion,
        m: i32,
    }

    let mut v1 = V {
        union_data: VUnion {
            inner: Inner { i: 0, j: 0 },
        },
        m: 0,
    };

    // SAFETY: We're writing to the `inner` field and then reading from `inner.i`
    unsafe {
        v1.union_data.inner.i = 2;
    }

    // SAFETY: We're writing to the `w` field which shares memory with `inner`
    unsafe {
        v1.union_data.w.k = 5;
    }

    // SAFETY: Reading from `inner.i` after writing to `w.k` is valid in C unions
    unsafe {
        if v1.union_data.inner.i != 2 {
            std::process::exit(1);
        }
    }

    // SAFETY: Reading from `w.k` after writing to it
    unsafe {
        if v1.union_data.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}