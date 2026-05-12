type t = i32;
type plain = i32;

#[derive(Clone, Copy)]
struct Tag {
    t: u8,
    r: u8,
}

fn id(x: t) -> t {
    x
}

fn call_cb(cb: fn(t) -> t, x: t) -> t {
    cb(x)
}

fn use_bits(s: Tag) -> t {
    let a = (s.t & 0x0f) as t;
    let b = (s.r & 0x1f) as t;
    a * 100 + b
}

fn f(pf: fn(t) -> t) -> t {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r: 0 };

    s.t = (15u32 & 0x0f) as u8;
    s.r = (31u32 & 0x1f) as u8;

    if use_bits(s) != (15 * 100 + 31) {
        std::process::exit(1);
    }

    {
        let t: i64 = 1234;

        if t != 1234 {
            std::process::exit(2);
        }

        if call_cb(id, 7) != 7 {
            std::process::exit(3);
        }

        if f(id) != 9 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}