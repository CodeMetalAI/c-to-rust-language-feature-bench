use std::process::exit;

type T = i32;

#[derive(Copy, Clone)]
struct Tag {
    t: u8,
    r: u8,
}

fn id(x: T) -> T {
    x
}

fn call_cb(cb: fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.t as T;
    let b = s.r as T;
    a * 100 + b
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r: 0 };

    s.t = 15u8 & 0xF;
    s.r = 31u8 & 0x1F;

    if use_bits(s) != (15 * 100 + 31) as T {
        exit(1);
    }

    {
        let mut t_var: i64 = 1234;

        if t_var != 1234 {
            exit(2);
        }

        if call_cb(id, 7) != 7 {
            exit(3);
        }

        if f(id) != 9 {
            exit(4);
        }
    }
}