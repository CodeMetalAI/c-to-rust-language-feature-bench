fn main() {
    struct V {
        i: i32,
        k: i64,
        m: i32,
    }

    let mut v1 = V { i: 0, k: 0, m: 0 };
    v1.i = 2;
    v1.k = 5;

    if v1.i!= 2 {
        std::process::exit(1);
    }

    if v1.k!= 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}