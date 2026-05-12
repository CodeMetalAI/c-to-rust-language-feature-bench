struct V {
    union U {
        struct W {
            i: i32,
            j: i32,
        },
        struct X {
            k: i64,
            l: i64,
        },
    },
    m: i32,
}

fn main() {
    let mut v1 = V {
        U: W { i: 2, j: 0 },
        m: 0,
    };

    v1.U.W.i = 2;
    v1.U.X.k = 5;

    if v1.U.W.i != 2 {
        return 1;
    }

    if v1.U.X.k != 5 {
        return 1;
    }

    return 0;
}