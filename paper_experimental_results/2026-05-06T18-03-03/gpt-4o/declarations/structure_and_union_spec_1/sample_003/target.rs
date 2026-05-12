struct V {
    w: W,
    m: i32,
}

union W {
    i: i32,
    j: i32,
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        w: W { i: 0 },
        m: 0,
    };
    
    unsafe {
        v1.w.i = 2;
        v1.w.k = 5;

        if v1.w.i != 2 {
            std::process::exit(1);
        }

        if v1.w.k != 5 {
            std::process::exit(1);
        }
    }
}