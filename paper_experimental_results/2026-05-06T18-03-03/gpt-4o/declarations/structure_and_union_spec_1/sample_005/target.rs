struct V {
    i: i32,
    j: i32,
    m: i32,
}

fn main() {
    let mut v1 = V { i: 0, j: 0, m: 0 };
    v1.i = 2;
    v1.j = 0; // This is necessary to ensure the union-like behavior
    let k = 5;
    let l = 0; // This is necessary to ensure the union-like behavior

    if v1.i != 2 {
        std::process::exit(1);
    }

    if k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}