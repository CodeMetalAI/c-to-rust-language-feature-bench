#[repr(C)]
struct V {
    data: [u8; 20],
}

impl V {
    fn set_i(&mut self, val: i32) {
        self.data[0..4].copy_from_slice(&val.to_le_bytes());
    }

    fn get_i(&self) -> i32 {
        i32::from_le_bytes(self.data[0..4].try_into().unwrap())
    }

    fn set_w_k(&mut self, val: i64) {
        self.data[0..8].copy_from_slice(&val.to_le_bytes());
    }

    fn get_w_k(&self) -> i64 {
        i64::from_le_bytes(self.data[0..8].try_into().unwrap())
    }
}

fn main() {
    let mut v1 = V { data: [0; 20] };
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