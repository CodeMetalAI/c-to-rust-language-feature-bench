// type_def_3.rs

type T = i32;
type Plain = i32;

struct Tag {
    t: u8, // 4 bits
    r: Plain, // 5 bits
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

    s.t = 15;
    s.r = 31;

    if use_bits(s) != 15 * 100 + 31 {
        std::process::exit(1);
    }

    {
        let f = f as fn(fn(T) -> T) -> T;
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