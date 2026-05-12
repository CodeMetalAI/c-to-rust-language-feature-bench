struct V {
    i: i32,
    j: i32,
    w: W,
    m: i32,
}

union W {
    i: i32,
    l: i64,
}

fn main() {
    let mut v1 = V {
        i: 2,
        j: 0,
        w: W { l: 0 },
        m: 0,
    };
    v1.w.l = 5;

    if v1.i != 2 {
        return 1 as i32;
    }

    if v1.w.l != 5 {
        return 1 as i32;
    }

    0 as i32
}