fn main() {
    struct Inner {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    union Union {
        inner: Inner,
        w: W,
    }

    struct V {
        union_field: Union,
        m: i32,
    }

    let mut v1 = V {
        union_field: Union {
            inner: Inner { i: 0, j: 0 },
        },
        m: 0,
    };

    // SAFETY: We are writing to the `inner` variant and then reading from it
    unsafe {
        v1.union_field.inner.i = 2;
    }

    // SAFETY: We are writing to the `w` variant, but we know the union layout
    // allows this because we just initialized with `inner` and are now overwriting
    unsafe {
        v1.union_field.w.k = 5;
    }

    // SAFETY: We are reading from the `inner` variant after writing to `w`
    // This is allowed because unions in C/C++ allow type punning
    unsafe {
        if v1.union_field.inner.i != 2 {
            std::process::exit(1);
        }
    }

    // SAFETY: We are reading from the `w` variant after writing to it
    unsafe {
        if v1.union_field.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}