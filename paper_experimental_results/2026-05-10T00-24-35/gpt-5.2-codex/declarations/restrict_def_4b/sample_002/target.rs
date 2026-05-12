use std::process::exit;

type USize = u64;

const POOL_SIZE_BYTES: usize = 8192;
const FLOAT_SIZE_BYTES: usize = 4;
const POOL_SIZE_FLOATS: usize = POOL_SIZE_BYTES / FLOAT_SIZE_BYTES;

const CONST1: USize = 0x9e3779b97f4a7c15;
const CONST2: USize = 0x5851f42d4c957f2d;

struct Vector {
    n: i32,
    v: Option<usize>, // byte offset into pool
}

struct Allocator {
    pool: Vec<f32>,
    off_bytes: usize,
    h: USize,
}

impl Allocator {
    fn new() -> Self {
        Self {
            pool: vec![0.0f32; POOL_SIZE_FLOATS],
            off_bytes: 0,
            h: CONST1,
        }
    }

    fn alloc_bytes(&mut self, n: usize) -> Option<usize> {
        if n == 0 {
            return Some(self.off_bytes);
        }
        if self.off_bytes + n > POOL_SIZE_BYTES {
            return None;
        }
        let p = self.off_bytes;
        self.off_bytes += n;

        let p_u = p as USize;
        self.h ^= p_u.wrapping_add(CONST1);
        self.h = self.h.wrapping_mul(CONST2);

        Some(p)
    }
}

fn new_vector(n: i32, alloc: &mut Allocator) -> Vector {
    let bytes = n as usize * FLOAT_SIZE_BYTES;
    let v = alloc.alloc_bytes(bytes);
    Vector { n, v }
}

fn fill_vec(alloc: &mut Allocator, p: usize, n: i32, base: f32) {
    let start = p / FLOAT_SIZE_BYTES;
    let mut i = 0;
    while i < n {
        alloc.pool[start + i as usize] = base + i as f32;
        i += 1;
    }
}

fn sum_vec(alloc: &Allocator, p: usize, n: i32) -> f32 {
    let start = p / FLOAT_SIZE_BYTES;
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        s += alloc.pool[start + i as usize];
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0 {
        d = -d;
    }
    d < 0.0001
}

fn main() {
    let mut alloc = Allocator::new();

    let a = new_vector(16, &mut alloc);
    let b = new_vector(16, &mut alloc);

    if a.v.is_none() {
        exit(1);
    }
    if b.v.is_none() {
        exit(2);
    }

    if a.v == b.v {
        exit(3);
    }

    fill_vec(&mut alloc, a.v.unwrap(), a.n, 1.0f32);
    fill_vec(&mut alloc, b.v.unwrap(), b.n, 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(&alloc, a.v.unwrap(), a.n), expected_a) {
        exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(&alloc, b.v.unwrap(), b.n), expected_b) {
        exit(5);
    }

    if (alloc.h & 1u64) == 0u64 {
        exit(6);
    }

    exit(0);
}