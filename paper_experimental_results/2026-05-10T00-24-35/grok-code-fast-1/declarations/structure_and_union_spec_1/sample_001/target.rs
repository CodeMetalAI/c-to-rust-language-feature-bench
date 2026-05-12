#[derive(Clone, Copy)]
struct V {
    shared: i64,
    m: i32,
}

impl V {
    fn new() -> Self {
        V { shared: 0, m: 0 }
    }

    fn set_i(&mut self, val: i32) {
        // Set the lower 32 bits to the bits of val
        self.shared = (self.shared & (-1i64 << 32)) | ((val as u32) as i64);
    }

    fn get_i(&self) -> i32 {
        self.shared as i32
    }

    fn set_w_k(&mut self, val: i64) {
        self.shared = val;
    }

    fn get_w_k(&self) -> i64 {
        self.shared
    }
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2);
    v1.set_w_k(5);

    if v1.get_i() != 2 {
        std::process::exit(1);
    }

    if v1.get_w_k() != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}