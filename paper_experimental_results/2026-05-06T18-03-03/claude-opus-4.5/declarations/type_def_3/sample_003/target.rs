struct Tag {
    t: u8,
    r: i8,
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: &Tag) -> i32 {
    let a: i32 = s.t as i32;
    let b: i32 = s.r as i32;
    (a * 100 + b) as i32
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 0, r: 0 };

    s.t = 15u8 & 0x0F;
    s.r = 31i8 & 0x1F;

    if use_bits(&s) != (15 * 100 + 31) as i32 {
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
}