#[derive(Debug)]
struct v {
    union: Union,
    m: i32,
}

#[derive(Debug)]
union Union {
    inner1: Inner1,
    w: w,
}

#[derive(Debug)]
struct Inner1 {
    i: i32,
    j: i32,
}

#[derive(Debug)]
struct w {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = v {
        union: Union {
            inner1: Inner1 { i: 0, j: 0 },
            w: w { k: 0, l: 0 },
        },
        m: 0,
    };

    v1.union.inner1.i = 2;
    v1.union.w.k = 5;

    if v1.union.inner1.i != 2 {
        return 1;
    }

    if v1.union.w.k != 5 {
        return 1;
    }

    0
}