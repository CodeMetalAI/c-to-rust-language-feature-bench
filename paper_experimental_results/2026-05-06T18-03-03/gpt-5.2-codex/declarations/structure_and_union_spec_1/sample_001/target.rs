use std::process::exit;

struct V {
    data: [u8; 16],
    m: i32,
}

impl V {
    fn new() -> Self {
        Self {
            data: [0u8; 16],
            m: 0,
        }
    }

    fn set_i(&mut self, val: i32) {
        self.data[0..4].copy_from_slice(&val.to_ne_bytes());
    }

    fn set_k(&mut self, val: i64) {
        self.data[0..8].copy_from_slice(&val.to_ne_bytes());
    }

    fn get_i(&self) -> i32 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&self.data[0..4]);
        i32::from_ne_bytes(arr)
    }

    fn get_k(&self) -> i64 {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&self.data[0..8]);
        i64::from_ne_bytes(arr)
    }
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2);
    v1.set_k(5);

    let code = if v1.get_i() != 2 {
        1
    } else if v1.get_k() != 5 {
        1
    } else {
        0
    };

    exit(code);
}