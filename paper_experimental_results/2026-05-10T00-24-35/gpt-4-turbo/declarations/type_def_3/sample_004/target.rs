struct Tag {
    t: u8, // 4 bits
    r: i8, // 5 bits
}

impl Tag {
    fn new(t: u8, r: i8) -> Self {
        Tag {
            t: t & 0b1111, // Mask to 4 bits
            r: r & 0b11111, // Mask to 5 bits
        }
    }
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
    let a = s.t as i32; // u8 to i32
    let b = s.r as i32; // i8 to i32
    a * 100 + b
}

fn f<F>(pf: F) -> i32
where
    F: Fn(i32) -> i32,
{
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag::new(15, 31);

    if use_bits(&s) != (15 * 100 + 31) {
        std::process::exit(1);
    }

    let t = 1234i64; // `long` in C++, equivalent to `i64` in Rust

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