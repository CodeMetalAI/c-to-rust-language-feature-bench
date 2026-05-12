struct UnionU {
    bytes: [u8; 4],
}

impl UnionU {
    fn new() -> Self {
        Self { bytes: [0; 4] }
    }

    fn set_any_member(&mut self, val: i32) {
        self.bytes = val.to_ne_bytes();
    }

    fn get_any_member(&self) -> i32 {
        i32::from_ne_bytes(self.bytes)
    }

    fn set_u_member(&mut self, val: u32) {
        self.bytes = val.to_ne_bytes();
    }

    fn get_u_member(&self) -> u32 {
        u32::from_ne_bytes(self.bytes)
    }
}

fn main() {
    let mut x = UnionU::new();
    x.set_any_member(42);

    if x.get_any_member() != 42 {
        std::process::exit(1);
    }

    {
        let mut y = UnionU::new();
        y.set_u_member(7);
        if y.get_u_member() != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}