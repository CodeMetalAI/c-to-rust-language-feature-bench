type T = i32;
type Plain = i32;

#[derive(Clone, Copy)]
struct Tag(u16);

impl Tag {
    fn new() -> Self {
        Tag(0)
    }

    fn set_t(&mut self, val: u32) {
        self.0 = (self.0 & !0xF) | (val & 0xF);
    }

    fn get_t(&self) -> T {
        (self.0 & 0xF) as T
    }

    fn set_r(&mut self, val: Plain) {
        let mask = 0x1E00;
        let shift = 9;
        self.0 = (self.0 & !mask) | (((val as u16) & 0x1F) << shift);
    }

    fn get_r(&self) -> T {
        let shift = 9;
        (((self.0 >> shift) & 0x1F) as i32)
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

fn main() {
    let mut s = Tag::new();

    s.set_t(15);
    s.set_r(31);

    if use_bits(s) != (15 * 100 + 31) {
        std::process::exit(1);
    }

    {
        let _f: fn(fn(T) -> T) -> T = f;
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