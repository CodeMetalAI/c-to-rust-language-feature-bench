type T = i32;
type Plain = i32;

#[derive(Clone, Copy)]
struct Tag {
    value: u16,
}

impl Tag {
    fn new() -> Self {
        Tag { value: 0 }
    }

    fn t(&self) -> u8 {
        (self.value & 0b1111) as u8
    }

    fn set_t(&mut self, val: u8) {
        self.value = (self.value & !0b1111) | (val as u16 & 0b1111);
    }

    fn r(&self) -> u8 {
        ((self.value >> 9) & 0b11111) as u8
    }

    fn set_r(&mut self, val: u8) {
        self.value = (self.value & !(0b11111 << 9)) | ((val as u16 & 0b11111) << 9);
    }
}

fn id(x: T) -> T {
    x
}

fn call_cb<F: Fn(T) -> T>(cb: F, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a = s.t() as T;
    let b = s.r() as T;
    (a * 100 + b) as T
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag::new();
    s.set_t(15);
    s.set_r(31);
    let expected = (15i32 * 100 + 31) as T;
    if use_bits(s) != expected {
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