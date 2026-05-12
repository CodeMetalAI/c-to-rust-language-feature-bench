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

impl Default for W {
    fn default() -> Self {
        W { k: 0, l: 0 }
    }
}

impl Default for V {
    fn default() -> Self {
        V {
            i: 0,
            j: 0,
            w: W::default(),
            m: 0,
        }
    }
}

fn main() {
    let mut v1 = V::default();
    v1.i = 2;
    v1.w.k = 5;

    if v1.i != 2 {
        std::process::exit(1);
    }

    if v1.w.k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}