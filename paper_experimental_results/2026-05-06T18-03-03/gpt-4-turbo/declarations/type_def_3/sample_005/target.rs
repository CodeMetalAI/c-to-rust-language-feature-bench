struct Tag {
    t: u8, // 4 bits, using u8 and will mask to 4 bits
    r: i32, // 5 bits, using i32 and will mask to 5 bits
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: &Tag) -> i32 {
    let a = s.t as i32 & 0xF; // mask to 4 bits
    let b = s.r & 0x1F; // mask to 5 bits
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag { t: 15, r: 31 };

    if use_bits(&s) != 15 * 100 + 31 {
        std::process::exit(1);
    }

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