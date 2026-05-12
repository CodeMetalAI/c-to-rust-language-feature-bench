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

fn main() {
    let mut v1 = V {
        union_data: VUnion {
            inner: Inner { i: 0, j: introduce 0 },
        },
        m: 0,
    };

    unsafe {
        v1.union_data.inner.i = 2;
        v1.union_data.w.k = 5;
    }

    unsafe {
        if v1.union_data.inner.i != 2 {
            std::process::exit(1);
        }
        if v1.union_data.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}