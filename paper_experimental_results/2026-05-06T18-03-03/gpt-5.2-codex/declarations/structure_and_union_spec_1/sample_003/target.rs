use std::mem::size_of;

struct V {
    data: Vec<u8>,
    m: i32,
}

impl V {
    fn new() -> Self {
        let size_i = 2 * size_of::<i32>();
        let size_k = 2 * size_of::<isize>();
        let size = if size_i > size_k { size_i } else { size_k };
        V {
            data: vec![0u8; size],
            m: 0,
        }
    }

    fn set_i(&mut self, val: i32) {
        let bytes = val.to_ne_bytes();
        self.data[..size_of::<i32>()].copy_from_slice(&bytes);
    }

    fn get_i(&self) -> i32 {
        let mut bytes = [0u8; size_of::<i32>()];
        bytes.copy_from_slice(&self.data[..size_of::<i32>()]);
        i32::from_ne_bytes(bytes)
    }

    fn set_k(&mut self, val: isize) {
        let bytes = val.to_ne_bytes();
        let len = size_of::<isize>();
        self.data[..len].copy_from_slice(&bytes);
    }

    fn get_k(&self) -> isize {
        let mut bytes = [0u8; size_of::<isize>()];
        bytes.copy_from_slice(&self.data[..size_of::<isize>()]);
        isize::from_ne_bytes(bytes)
    }
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2);
    v1.set_k(5);

    if v1.get_i() != 2 {
        std::process::exit(1);
    }

    if v1.get_k() != 5 {
        std::process::exit(1);
    }
}