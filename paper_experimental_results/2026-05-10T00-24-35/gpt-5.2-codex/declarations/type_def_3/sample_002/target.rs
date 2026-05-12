use std::process::exit;

type T = i32;

#[derive(Copy, Clone)]
struct Tag {
    bits: u32,
}

impl Tag {
    fn new() -> Self {
        Tag { bits: 0 }
    }
    fn set_t(&mut self, val: u32) {
        self.bits = (self.bits & !0xF) | (val & 0xF);
    }
    fn set_r(&mut self, val: u32) {
        let offset = 9;
        let mask = 0x1F << offset;
        self.bits = (self.bits & !mask) | ((val & 0x1F) << offset);
    }
    fn get_t(&self) -> u32 {
        self.bits & 0xF
    }
    fn get_r(&self) -> u32 {
        (self.bits >> 9) & 0x1F
    }
}

fn id(x: T) -> T {
    x
}

fn call_cb(cb: fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.get_t() as T;
    let b = s.get_r() as T;
    a * 100 + b
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag::new();

    s.set_t(15);
    s.set_r(31);

    if use_bits(s) != 15 * 100 + 31 {
        exit(1);
    }

    {
        let t: i64 = 1234;

        if t != 1234 {
            exit(2);
        }

        if call_cb(id, 7) != 7 {
            exit(3);
        }

        if f(id) != 9 {
            exit(4);
        }
    }

    exit(0);
}