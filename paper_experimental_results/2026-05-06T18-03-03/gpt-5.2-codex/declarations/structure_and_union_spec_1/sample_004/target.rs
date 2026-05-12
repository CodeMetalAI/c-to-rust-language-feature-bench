use std::mem::size_of;
use std::os::raw::{c_int, c_long};

const CI_SIZE: usize = size_of::<c_int>();
const CL_SIZE: usize = size_of::<c_long>();
const UNION_SIZE: usize = if 2 * CI_SIZE > 2 * CL_SIZE {
    2 * CI_SIZE
} else {
    2 * CL_SIZE
};

struct V {
    union_bytes: [u8; UNION_SIZE],
    m: c_int,
}

impl V {
    fn new() -> Self {
        Self {
            union_bytes: [0u8; UNION_SIZE],
            m: 0,
        }
    }

    fn set_i(&mut self, val: c_int) {
        let bytes = val.to_ne_bytes();
        self.union_bytes[..CI_SIZE].copy_from_slice(&bytes);
    }

    fn set_k(&mut self, val: c_long) {
        let bytes = val.to_ne_bytes();
        self.union_bytes[..CL_SIZE].copy_from_slice(&bytes);
    }

    fn get_i(&self) -> c_int {
        let mut arr = [0u8; CI_SIZE];
        arr.copy_from_slice(&self.union_bytes[..CI_SIZE]);
        c_int::from_ne_bytes(arr)
    }

    fn get_k(&self) -> c_long {
        let mut arr = [0u8; CL_SIZE];
        arr.copy_from_slice(&self.union_bytes[..CL_SIZE]);
        c_long::from_ne_bytes(arr)
    }
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2 as c_int);
    v1.set_k(5 as c_long);

    if v1.get_i() != 2 {
        std::process::exit(1);
    }

    if v1.get_k() != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}