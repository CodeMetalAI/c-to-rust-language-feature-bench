fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() {
    let a = 0;
    let c = 0;
    let mut t: i32;

    let v = f(a, { t = 3; t + 2 }, c);

    if v != 5 {
        std::process::exit(1);
    }

    let mut t1: i32;
    if ({ t1 = 3; t + 2 }) != 5 {
        std::process::exit(2);
    }

    let mut t2: i32;
    let t3 = if 1 < 2 {
        { t2 = 0; t2 + 2 }
    } else {
        { t2 = 1; t2 + 2 }
    };
    if t3 != 2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}