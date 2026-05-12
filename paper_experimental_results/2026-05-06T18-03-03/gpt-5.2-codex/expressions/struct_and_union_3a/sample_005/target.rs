use std::process::exit;

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) / align * align
}

const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

const INT_SIZE: usize = std::mem::size_of::<i32>();
const DOUBLE_SIZE: usize = std::mem::size_of::<f64>();
const DOUBLE_ALIGN: usize = std::mem::align_of::<f64>();
const DOUBLE_OFFSET: usize = align_up(INT_SIZE, DOUBLE_ALIGN);
const NF_SIZE: usize = DOUBLE_OFFSET + DOUBLE_SIZE;
const UNION_SIZE: usize = max(INT_SIZE, max(2 * INT_SIZE, NF_SIZE));

struct UnionU {
    bytes: [u8; UNION_SIZE],
}

impl UnionU {
    fn new() -> Self {
        Self { bytes: [0u8; UNION_SIZE] }
    }

    fn write_i32(&mut self, offset: usize, value: i32) {
        let b = value.to_ne_bytes();
        self.bytes[offset..offset + INT_SIZE].copy_from_slice(&b);
    }

    fn write_f64(&mut self, offset: usize, value: f64) {
        let b = value.to_ne_bytes();
        self.bytes[offset..offset + DOUBLE_SIZE].copy_from_slice(&b);
    }

    fn read_i32(&self, offset: usize) -> i32 {
        let mut b = [0u8; INT_SIZE];
        b.copy_from_slice(&self.bytes[offset..offset + INT_SIZE]);
        i32::from_ne_bytes(b)
    }

    fn read_f64(&self, offset: usize) -> f64 {
        let mut b = [0u8; DOUBLE_SIZE];
        b.copy_from_slice(&self.bytes[offset..offset + DOUBLE_SIZE]);
        f64::from_ne_bytes(b)
    }

    fn set_nf_type(&mut self, value: i32) {
        self.write_i32(0, value);
    }

    fn set_nf_doublenode(&mut self, value: f64) {
        self.write_f64(DOUBLE_OFFSET, value);
    }

    fn get_n_alltypes(&self) -> i32 {
        self.read_i32(0)
    }

    fn get_ni_type(&self) -> i32 {
        self.read_i32(0)
    }

    fn get_nf_type(&self) -> i32 {
        self.read_i32(0)
    }

    fn get_nf_doublenode(&self) -> f64 {
        self.read_f64(DOUBLE_OFFSET)
    }
}

fn main() {
    let mut u = UnionU::new();

    u.set_nf_type(1);
    u.set_nf_doublenode(3.14);

    if u.get_n_alltypes() != 1 {
        exit(1);
    }
    if u.get_ni_type() != 1 {
        exit(1);
    }
    if u.get_nf_type() != 1 {
        exit(2);
    }
    if u.get_nf_doublenode() != 3.14 {
        exit(3);
    }

    exit(0);
}