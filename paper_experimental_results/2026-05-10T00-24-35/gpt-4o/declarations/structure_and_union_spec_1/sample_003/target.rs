struct V {
    inner: Inner,
    m: i32,
}

union Inner {
    int_pair: IntPair,
    long_pair: LongPair,
}

#[derive(Copy, Clone)]
struct IntPair {
    i: i32,
    j: i32,
}

#[derive(Copy, Clone)]
struct LongPair {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        inner: Inner {
            int_pair: IntPair { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.inner.int_pair.i = 2;
        v1.inner.long_pair.k = 5;

        if v1.inner.int_pair.i != 2 {
            std::process::exit(1);
        }

        if v1.inner.long_pair.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}