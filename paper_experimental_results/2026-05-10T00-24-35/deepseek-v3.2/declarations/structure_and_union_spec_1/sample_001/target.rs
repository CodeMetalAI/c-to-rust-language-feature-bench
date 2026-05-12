fn main() {
    struct Inner1 {
        i: i32,
        j: i32,
    }

    struct Inner2 {
        k: i64,
        l: i64,
    }

    struct V {
        union_data: UnionData,
        m: i32,
    }

    union UnionData {
        inner1: Inner1,
        inner2: Inner2,
    }

    let mut v1 = V {
        union_data: UnionData {
            inner1: Inner1 { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.union_data.inner1.i = 2;
        v1.union_data.inner2.k = 5;
    }

    let exit_code = unsafe {
        if v1.union_data.inner1.i != 2 {
            1
        } else if v1.union_data.inner2.k != 5 {
            1
        } else {
            0
        }
    };

    std::process::exit(exit_code);
}