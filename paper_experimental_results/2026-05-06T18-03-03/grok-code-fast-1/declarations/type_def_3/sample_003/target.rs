struct Tag {
    bits: u16,
}

impl Tag {
    fn new() -> Self {
        Tag { bits: 0 }
    }

    fn set_t(&mut self, val: u32) {
        self.bits = (self.bits & !0xF) | (val & 0xF);
    }

    fn set_r(&mut self, val: i32) {
        self.bits = (self.bits & !(0x1F << 9)) | (((val as u16) & 0x1F) << 9);
    }

    fn t(&self) -> u32 {
        self.bits & 0xF
    }

    fn r(&self) -> i32 {
        ((self.bits >> 9) & 0x1F) as i32
    }
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t() as i32;
    let b = s.r();
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag::new();
    s.set_t(15);
    s.set_r(31);

    if use_bits(s) != 15 * 100 + 31 {
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

    std::process::exit(0);
}