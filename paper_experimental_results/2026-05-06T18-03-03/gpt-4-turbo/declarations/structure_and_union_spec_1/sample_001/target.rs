fn main() {
    struct V {
        inner: Inner,
        m: i32,
    }

    union Inner {
        unnamed: Unnamed,
        w: W,
    }

    #[derive(Default)]
    struct Unnamed {
        i: i32,
        j: i32,
    }

    #[derive(Default)]
    struct W {
        k: i64,
        l: i64,
    }

    let mut v1 = V {
        inner: Inner {
            unnamed: Unnamed { i: 2, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.inner.w.k = 5;
    }

    let i_value = unsafe { v1.inner.unnamed.i };
    let k_value = unsafe { v1.inner.w.k };

    if i_value != 2 {
        std::process::exit(1);
    }

    if k_value != 5 {
        std::process::exit(1);
    }
}