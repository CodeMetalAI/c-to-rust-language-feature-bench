struct V {
    w: W,
    m: i32,
}

union W {
    ij: IJ,
    kl: KL,
}

struct IJ {
    i: i32,
    j: i32,
}

struct KL {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        w: W { ij: IJ { i: 2, j: 0 } },
        m: 0,
    };
    unsafe { v1.w.kl = KL { k: 5, l: 0 } };

    if unsafe { v1.w.ij.i } != 2 {
        std::process::exit(1);
    }

    if unsafe { v1.w.kl.k } != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}