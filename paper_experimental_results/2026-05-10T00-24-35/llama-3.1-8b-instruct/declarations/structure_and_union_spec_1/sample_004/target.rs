struct V {
    union W {
        struct X {
            i: i32,
            j: i32,
        },
        struct Y {
            k: i64,
            l: i64,
        },
    },
    m: i32,
}

fn main() {
    let mut v1 = V {
        W: W {
            X: X { i: 2, j: 0 },
            ..Default::default()
        },
        m: 0,
    };

    v1.W.X.i = 2;
    v1.W.Y.k = 5;

    if v1.W.X.i != 2 {
        std::process::exit(1);
    }

    if v1.W.Y.k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}