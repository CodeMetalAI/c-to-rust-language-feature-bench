fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() -> i32 {
    let a = 0;
    let c = 0;
    let t = 3;
    let v = f(a, t + 2, c);

    if v != 5 {
        return 1;
    }

    let t1 = 3;
    if t1 + 2 != 5 {
        return 2;
    }

    let t2;
    let t3 = if 1 < 2 {
        t2 = 0;
        t2 + 2
    } else {
        t2 = 1;
        t2 + 2
    };

    if t3 != 2 {
        return 3;
    }

    return 0;
}