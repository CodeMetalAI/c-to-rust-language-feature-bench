use std::process::exit;

#[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]
type CLong = i64;
#[cfg(any(target_os = "windows", not(target_pointer_width = "64")))]
type CLong = i32;

const INT_SIZE: usize = std::mem::size_of::<i32>();
const CLONG_SIZE: usize = std::mem::size_of::<CLong>();
const STORAGE_SIZE: usize = if 2 * INT_SIZE > 2 * CLONG_SIZE {
    2 * INT_SIZE
} else {
    2 * CLONG_SIZE
};

#[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]
fn clong_to_ne_bytes(x: CLong) -> [u8; 8] {
    x.to_ne_bytes()
}
#[cfg(any(target_os = "windows", not(target_pointer_width = "64")))]
fn clong_to_ne_bytes(x: CLong) -> [u8; 4] {
    x.to_ne_bytes()
}

#[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]
fn clong_from_ne_bytes(bytes: &[u8]) -> CLong {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(bytes);
    CLong::from_ne_bytes(arr)
}
#[cfg(any(target_os = "windows", not(target_pointer_width = "64")))]
fn clong_from_ne_bytes(bytes: &[u8]) -> CLong {
    let mut arr = [0u8; 4];
    arr.copy_from_slice(bytes);
    CLong::from_ne_bytes(arr)
}

struct V {
    storage: [u8; STORAGE_SIZE],
    m: i32,
}

impl V {
    fn new() -> Self {
        Self {
            storage: [0u8; STORAGE_SIZE],
            m: 0,
        }
    }

    fn set_i(&mut self, value: i32) {
        let bytes = value.to_ne_bytes();
        self.storage[0..INT_SIZE].copy_from_slice(&bytes);
    }

    fn get_i(&self) -> i32 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&self.storage[0..INT_SIZE]);
        i32::from_ne_bytes(arr)
    }

    fn set_w_k(&mut self, value: CLong) {
        let bytes = clong_to_ne_bytes(value);
        self.storage[0..CLONG_SIZE].copy_from_slice(&bytes);
    }

    fn get_w_k(&self) -> CLong {
        clong_from_ne_bytes(&self.storage[0..CLONG_SIZE])
    }
}

fn main() {
    let mut v1 = V::new();
    v1.set_i(2);
    v1.set_w_k(5);

    if v1.get_i() != 2 {
        exit(1);
    }

    if v1.get_w_k() != 5 {
        exit(1);
    }

    exit(0);
}