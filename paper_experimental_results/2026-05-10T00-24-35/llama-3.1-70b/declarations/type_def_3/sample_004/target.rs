fn main() {
    #[derive(Debug, Default)]
    struct Tag {
        t: u8,
        r: i8,
    }

    let id = |x: i32| x;

    let call_cb = |cb: impl Fn(i32) -> i32, x: i32| cb(x);

    let use_bits = |s: Tag| (s.t as i32 * 100 + s.r as i32) as i32;

    let f = |pf: impl Fn(i32) -> i32| call_cb(pf, 9);

    let mut s = Tag { t: 15, r: 31 };

    if use_bits(s)!= 1513 {
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