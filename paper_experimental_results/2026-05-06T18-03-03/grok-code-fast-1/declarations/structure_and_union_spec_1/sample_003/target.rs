struct V {
    data: [u8; 12],
}

impl V {
    fn set_i(&mut self, val: i32) {
        let bytes = val.to_le_bytes();
        self.data[0..4].copy_from_slice(&bytes);
    }

    fn get_i(&self) -> i32 {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.data[0..4]);
        i32::from_le_bytes(bytes)
    }

    fn set_w_k(&mut self, val: i64) {
        let bytes = val.to_le_bytes();
        self.data[0..8].copy_from_slice(&bytes);
    }

    fn get_w_k(&self) -> i64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[0..8]);
        i64::from_le_bytes(bytes)
    }
}

fn main() {
    let mut v1 = V { data: [0; 12] };
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