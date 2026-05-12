#[derive(Default)]
struct V {
    i: i32,
    j: i32,
    k: i64,
    l: i64,
    m: i32,
}

fn main() {
    let mut v1 = V::default();
    v1.i = 2;
    v1.k = 5;

    if v1.i != 2 {
        std::process::exit(1);
    }

    if v1.k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}