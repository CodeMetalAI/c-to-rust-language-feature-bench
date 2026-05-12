#[derive(Debug)]
struct v {
    union: Union,
    m: i32,
}

#[derive(Debug)]
union Union {
    inner1: Inner1,
    inner2: Inner2,
}

#[derive(Debug)]
struct Inner1 {
    i: i32,
    j: i32,
}

#[derive(Debug)]
struct Inner2 {
    k: i64,
    l: i64,
    w: Inner2Fields,
}

#[derive(Debug)]
struct Inner2Fields {
    k: i64,
}

fn main() {
    let mut v1 = v {
        union: Union {
            inner1: Inner1 { i: 0, j: 0 },
            inner2: Inner2 { k: 0, l: 0, w: Inner2Fields { k: 0 } },
        },
        m: 0,
    };

    v1.union.inner1.i = 2;
    v1.union.inner2.k = 5;

    if v1.union.inner1.i != 2 {
        return 1;
    }

    if v1.union.inner2.k != 5 {
        return 1;
    }

    0
}