struct Tag {
    t: u8, // 4 bits, use a full byte but only utilize 4 bits
    r: u8, // 5 bits, use a full byte but only utilize 5 bits
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb<F>(cb: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    cb(x)
}

fn use_bits(s: &Tag) -> i32 {
    let a = s.t as i32; // only the lower 4 bits are used, safe to cast
    let b = s.r as i32; // only the lower 5 bits are used, safe to cast
    a * 100 + b
}

fn f<F>(pf: F) -> i32
where
    F: Fn(i32) -> i32,
{
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r: 0 };

    s.t = 15; // only the lower 4 bits are used
    s.r = 31; // only the lower 5 bits are used

    if use_bits(&s) != (15 * 100 + 31) {
        std::process::exit(1);
    }

    let mut t = 1234i64;
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