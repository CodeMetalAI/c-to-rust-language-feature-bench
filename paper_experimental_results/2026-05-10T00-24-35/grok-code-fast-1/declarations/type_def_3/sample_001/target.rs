type T = i32;
type Plain = i32;

#[derive(Clone, Copy)]
struct Tag(u16);

impl Tag {
    fn new() -> Self {
        Tag(0)
    }
    fn set_t(&mut self, value: u32) {
        self.0 = (self.0 & !0xf) | (value & 0xf);
    }
    fn get_t(&self) -> u32 {
        self.0 & 0xf
    }
    fn set_r(&mut self, value: Plain) {
        self.0 = (self.0 & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
    }
    fn get_r(&self) -> Plain {
        ((self.0 >> 9) & 0x1f) as Plain
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

fn main() -> i32 {
    let mut s = Tag::new();
    s.set_t(15);
    s.set_r(31);

    if use_bits(s) != (15 * 100 + 31) {
        return 1;
    }

    {
        let t: i64 = 1234;
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