fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() {
    let mut a = 0;
    let mut c = 0;
    let mut t = 3;
    t += 2;

    let v = f(a, t, c);

    if v != 5 {
        std::process::exit(1);
    }

    let mut t1 = 3;
    t1 += 2;
    if t1 != 5 {
        std::process::exit(2);
    }

    let mut t2;
    let t3 = if 1 < 2 {
        t2 = 0;
        t2 + 2
    } else {
        t2 = 1;
        t2 + 2
    };

    if t3 != 2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}