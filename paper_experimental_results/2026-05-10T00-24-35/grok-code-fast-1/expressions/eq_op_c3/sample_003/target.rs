fn eval_chained(a: i32, b: i32, c: i32, d: i32) -> bool {
    ((a < if b == c { 1 } else { 0 }) as i32) < d
}

fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;
    let mut d: i32 = 4;
    if !eval_chained(a, b, c, d) {
        std::process::exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if !eval_chained(a, b, c, d) {
        std::process::exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if eval_chained(a, b, c, d) {
        std::process::exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if eval_chained(a, b, c, d) {
        std::process::exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if !((a < b) == (c < d)) {
        std::process::exit(5);
    }

    std::process::exit(0);
}