struct Tag {
    t: u8, // 4 bits, unsigned
    r: i8, // 5 bits, signed
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32; // Convert 4-bit unsigned to i32
    let b = s.r as i32; // Convert 5-bit signed to i32
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 15, r: 31 };

    if use_bits(s) != 15 * 100 + 31 {
        std::process::exit(1);
    }

    let t = 1234i64;

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