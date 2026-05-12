use std::process::exit;
use std::sync::{Mutex, OnceLock};

type Usize = usize;

const POOL_BYTES: usize = 8192;
const POOL_FLOATS: usize = POOL_BYTES / 4;

const H_INIT: u64 = 0x9e3779b97f4a7c15;
const H_XOR: u64 = 0x9e3779b97f4a7c15;
const H_MUL: u64 = 0x5851f42d4c957f2d;

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: Option<usize>, // address within pool
}

struct Pool {
    data: [f32; POOL_FLOATS],
    off: usize, // bytes
    h: u64,
}

impl Pool {
    fn base_addr(&self) -> usize {
        self.data.as_ptr() as usize
    }
}

static POOL: OnceLock<Mutex<Pool>> = OnceLock::new();

fn pool() -> &'static Mutex<Pool> {
    POOL.get_or_init(|| {
        Mutex::new(Pool {
            data: [0.0; POOL_FLOATS],
            off: 0,
            h: H_INIT,
        })
    })
}

fn alloc_bytes(n: Usize) -> Option<usize> {
    let mut p = pool().lock().unwrap();
    let base = p.base_addr();

    if n == 0 {
        return Some(base + p.off);
    }

    if p.off + n > POOL_BYTES {
        return None;
    }

    let addr = base + p.off;
    p.off += n;

    let mut h = p.h;
    h ^= (addr as u64).wrapping_add(H_XOR);
    h = h.wrapping_mul(H_MUL);
    p.h = h;

    Some(addr)
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    Vector {
        n,
        v: alloc_bytes(bytes),
    }
}

fn fill_vec(addr: usize, n: i32, base: f32) {
    let mut p = pool().lock().unwrap();
    let base_addr = p.base_addr();
    let offset = addr - base_addr;
    let start = offset / 4;
    let count = n as usize;
    for i in 0..count {
        p.data[start + i] = base + i as f32;
    }
}

fn sum_vec(addr: usize, n: i32) -> f32 {
    let p = pool().lock().unwrap();
    let base_addr = p.base_addr();
    let offset = addr - base_addr;
    let start = offset / 4;
    let count = n as usize;
    let mut s = 0.0f32;
    for i in 0..count {
        s += p.data[start + i];
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
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        exit(1);
    }
    if b.v.is_none() {
        exit(2);
    }

    let a_addr = a.v.unwrap();
    let b_addr = b.v.unwrap();

    if a_addr == b_addr {
        exit(3);
    }

    fill_vec(a_addr, a.n, 1.0);
    fill_vec(b_addr, b.n, 100.0);

    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(a_addr, a.n), expected_a) {
        exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(b_addr, b.n), expected_b) {
        exit(5);
    }

    let h = pool().lock().unwrap().h;
    if (h & 1u64) == 0 {
        exit(6);
    }

    exit(0);
}