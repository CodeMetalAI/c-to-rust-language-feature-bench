type T = i32;

#[derive(Clone, Copy)]
struct Tag {
    bits: u16,
}

impl Tag {
    fn new() -> Self {
        Tag { bits: 0 }
    }

    fn set_t(&mut self, val: u8) {
        self.bits = (self.bits & !(0xF)) | ((val & 0xF) as u16);
    }

    fn get_t(&self) -> u8 {
        (self.bits & 0xF) as u8
    }

    fn set_r(&mut self, val: T) {
        self.bits = (self.bits & !(0x1F << 9)) | (((val & 31) as u16) << 9);
    }

    fn get_r(&self) -> T {
        (((self.bits >> 9) & 31) as T)
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
    (a * 100 + b) as T
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag::new();
    s.set_t(15);
    s.set_r(31);
    if use_bits(s) != (15i32 * 100 + 31) {
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