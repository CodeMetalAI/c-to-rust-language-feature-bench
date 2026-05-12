fn main() {
    struct V {
        inner: Inner,
        m: i32,
    }

    union Inner {
        unnamed: Unnamed,
        w: W,
    }

    struct Unnamed {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    let mut v1 = V {
        inner: Inner {
            unnamed: Unnamed { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.inner.unnamed.i = 2;
        v1.inner.w.k = 5;
    }

    unsafe {
        if v1.inner.unnamed.i != 2 {
            std::process::exit(1);
        }

        if v1.inner.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}