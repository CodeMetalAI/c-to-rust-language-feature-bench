fn main() {
    #[derive(Default)]
    struct W {
        k: i64,
        l: i64,
    }

    #[derive(Default)]
    struct V {
        i: i32,
        j: i32,
        w: W,
        m: i32,
    }

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