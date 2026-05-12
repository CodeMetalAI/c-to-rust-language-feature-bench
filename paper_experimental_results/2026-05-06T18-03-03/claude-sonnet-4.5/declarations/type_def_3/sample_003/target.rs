type T = i32;
type Plain = i32;

struct Tag {
    t: u8,
    r: i8,
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
    (a * 100 + b) as T
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r: 0 };

    s.t = 15u8;
    s.r = 31i8;

    if use_bits(s) != (15 * 100 + 31) as T {
        std::process::exit(1);
    }

    {
        let t: i64 = 1234i64;

        if t != 1234i64 {
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