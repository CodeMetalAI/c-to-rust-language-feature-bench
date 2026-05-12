struct Tag {
    t: u8,
    r: i8,
}

type T = i32;

fn id(x: T) -> T {
    x
}

fn call_cb(cb: impl Fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.t as T;
    let b = s.r as T;
    a * 100 + b
}

fn f(pf: impl Fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 15, r: 31 };

    if use_bits(s)!= 15 * 100 + 31 {
        return;
    }

    let t: i64 = 1234;

    if t!= 1234 {
        return;
    }

    if call_cb(id, 7)!= 7 {
        return;
    }

    if f(id)!= 9 {
        return;
    }
}