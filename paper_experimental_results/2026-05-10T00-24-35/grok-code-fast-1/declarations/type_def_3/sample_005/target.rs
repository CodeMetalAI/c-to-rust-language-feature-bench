type T = i32;
type Plain = i32;

#[derive(Clone, Copy)]
struct Tag {
    t: u8,
    r_bits: u8,
}

fn id(x: T) -> T {
    x
}

fn call_cb(cb: fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.t as T;
    let b = if s.r_bits & 0x10 != 0 {
        (s.r_bits as T) - 32
    } else {
        s.r_bits as T
    };
    a * 100 + b
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r_bits: 0 };
    s.t = (15u32 & 0xf) as u8;
    s.r_bits = (31u32 & 0x1f) as u8;
    if use_bits(s) != (15 * 100 + 31) as T {
        std::process::exit(1);
    }
    {
        let mut t: i64 = 1234;
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