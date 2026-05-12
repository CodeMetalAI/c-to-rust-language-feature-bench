use std::mem::size_of;
use std::os::raw::{c_int, c_long};
use std::process::exit;

type CInt = c_int;
type CLong = c_long;

const INT_SIZE: usize = size_of::<CInt>();
const LONG_SIZE: usize = size_of::<CLong>();
const UNION_SIZE: usize = if INT_SIZE * 2 > LONG_SIZE * 2 {
    INT_SIZE * 2
} else {
    LONG_SIZE * 2
};

struct V {
    storage: [u8; UNION_SIZE],
    m: CInt,
}

impl V {
    fn new() -> Self {
        V {
            storage: [0u8; UNION_SIZE],
            m: 0,
        }
    }

    fn set_i(&mut self, val: CInt) {
        write_cint(&mut self.storage, val);
    }

    fn get_i(&self) -> CInt {
        read_cint(&self.storage)
    }

    fn set_w_k(&mut self, val: CLong) {
        write_clong(&mut self.storage, val);
    }

    fn get_w_k(&self) -> CLong {
        read_clong(&self.storage)
    }
}

fn write_cint(buf: &mut [u8], val: CInt) {
    let bytes = val.to_ne_bytes();
    buf[..bytes.len()].copy_from_slice(&bytes);
}

fn read_cint(buf: &[u8]) -> CInt {
    let mut bytes = [0u8; size_of::<CInt>()];
    bytes.copy_from_slice(&buf[..bytes.len()]);
    CInt::from_ne_bytes(bytes)
}

fn write_clong(buf: &mut [u8], val: CLong) {
    let bytes = val.to_ne_bytes();
    buf[..bytes.len()].copy_from_slice(&bytes);
}

fn read_clong(buf: &[u8]) -> CLong {
    let mut bytes = [0u8; size_of::<CLong>()];
    bytes.copy_from_slice(&buf[..bytes.len()]);
    CLong::from_ne_bytes(bytes)
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2 as CInt);
    v1.set_w_k(5 as CLong);

    if v1.get_i() != 2 as CInt {
        exit(1);
    }

    if v1.get_w_k() != 5 as CLong {
        exit(1);
    }
}