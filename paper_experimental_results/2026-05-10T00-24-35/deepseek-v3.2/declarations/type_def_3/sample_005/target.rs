#[allow(non_snake_case)]
fn main() {
    type t = i32;
    type plain = i32;

    struct Tag {
        t: u32,
        r: i32,
    }

    fn id(x: t) -> t {
        x
    }

    fn call_cb(cb: fn(t) -> t, x: t) -> t {
        cb(x)
    }

    fn use_bits(s: Tag) -> t {
        let a = s.t as t;
        let b = s.r as t;
        a * 100 + b
    }

    fn f(pf: fn(t) -> t) -> t {
        call_cb(pf, 9)
    }

    let s = Tag {
        t: 15u32,
        r: 31,
    };

    if use_bits(s) != (15 * 100 + 31) {
        return std::process::exit(1);
    }

    let t: i64 = 1234;

    if t != 1234 {
        return std::process::exit(2);
    }

    if call_cb(id, 7) != 7 {
        return std::process::exit(3);
    }

    if f(id) != 9 {
        return std::process::exit(4);
    }

    std::process::exit(0);
}