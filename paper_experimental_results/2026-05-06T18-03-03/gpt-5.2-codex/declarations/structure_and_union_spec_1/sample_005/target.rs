use std::process::exit;

struct W {
    k: i64,
    l: i64,
}

struct V {
    i: i32,
    j: i32,
    w: W,
    m: i32,
}

fn main() {
    let mut v1 = V {
        i: 0,
        j: 0,
        w: W { k: 0, l: 0 },
        m: 0,
    };

    v1.i = 2;
    v1.w.k = 5;

    if v1.i != 2 {
        exit(1);
    }

    if v1.w.k != 5 {
        exit(1);
    }

    exit(0);
}