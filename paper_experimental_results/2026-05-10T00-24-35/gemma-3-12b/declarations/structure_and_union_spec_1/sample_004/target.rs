#[derive(Debug)]
struct v {
    union_data: UnionData,
    m: i32,
}

#[derive(Debug)]
union UnionData {
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
    w: Inner2Wrapper,
}

#[derive(Debug)]
struct Inner2Wrapper {
    k: i64,
}

fn main() {
    let mut v1 = v {
        union_data: UnionData {
            inner1: Inner1 { i: 0, j: 0 },
            inner2: Inner2 { k: 0, l: 0, w: Inner2Wrapper{k: 0} },
        },
        m: 0,
    };

    v1.union_data.inner1.i = 2;
    v1.union_data.inner2.k = 5;

    if v1.union_data.inner1.i != 2 {
        return 1;
    }

    if v1.union_data.inner2.k != 5 {
        return 1;
    }

    0
}