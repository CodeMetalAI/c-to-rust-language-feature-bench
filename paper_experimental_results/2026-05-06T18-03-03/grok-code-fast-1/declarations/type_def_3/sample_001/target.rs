type T = i32;
type Plain = i32;

#[derive(Clone, Copy)]
struct Tag {
    bits: u16,
}

impl Tag {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn set_t(&mut self, val: u32) {
        self.bits = (self.bits & !0xF) | (val as u16 & 0xF);
    }

    fn get_t(&self) -> T {
        (self.bits & 0xF) as T
    }

    fn set_r(&mut self, val: T) {
        self.bits = (self.bits & !(0x1F << 9)) | ((val as u16 & 0x1F) << 9);
    }

    fn get_r(&self) -> T {
        ((self.bits >> 9) & 0x1F) as T
    }
}

fn id(x: T) -> T {
    x
}

fn call_cb(cb: fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.get_t();
    let b = s.get_r();
    a * 100 + b
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() -> i32 {
    let mut s = Tag::new();
    s.set_t(15);
    s.set_r(31);
    if use_bits(s) != 15 * 100 + 31 {
        return 1;
    }
    {
        let mut t: i64 = 1234;
        if t != 1234 {
            return 2;
        }
        if call_cb(id, 7) != 7 {
            return 3;
        }
        if f(id) != 9 {
            return 4;
        }
    }
    0
}