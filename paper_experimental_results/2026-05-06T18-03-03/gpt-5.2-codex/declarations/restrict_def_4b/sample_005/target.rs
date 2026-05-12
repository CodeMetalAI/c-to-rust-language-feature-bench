use std::sync::Mutex;

type USize = usize;

const POOL_BYTES: usize = 8192;
const POOL_FLOATS: usize = POOL_BYTES / 4;
const H_INIT: u64 = 0x9e3779b97f4a7c15;
const CONST_ADD: u64 = 0x9e3779b97f4a7c15;
const CONST_MUL: u64 = 0x5851f42d4c957f2d;

struct Global {
    pool: [f32; POOL_FLOATS],
    off: usize,
    h: u64,
}

static GLOBAL: Mutex<Global> = Mutex::new(Global {
    pool: [0.0; POOL_FLOATS],
    off: 0,
    h: H_INIT,
});

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: Option<usize>, // offset in bytes
}

fn alloc_bytes(n: USize) -> Option<usize> {
    let mut g = GLOBAL.lock().unwrap();

    if n == 0 {
        return Some(g.off);
    }

    if g.off + n > POOL_BYTES {
        return None;
    }

    let p = g.off;
    g.off += n;

    let p_u = p as u64;
    let mut h = g.h;
    h = h ^ p_u.wrapping_add(CONST_ADD);
    h = h.wrapping_mul(CONST_MUL);
    g.h = h;

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    let v = alloc_bytes(bytes);
    Vector { n, v }
}

fn fill_vec(p: usize, n: i32, base: f32) {
    let mut g = GLOBAL.lock().unwrap();
    let start = p / 4;
    let len = n as usize;
    let slice = &mut g.pool[start..start + len];
    let mut i = 0;
    while i < n {
        slice[i as usize] = base + i as f32;
        i += 1;
    }
}

fn sum_vec(p: usize, n: i32) -> f32 {
    let g = GLOBAL.lock().unwrap();
    let start = p / 4;
    let len = n as usize;
    let slice = &g.pool[start..start + len];
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        s += slice[i as usize];
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
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        std::process::exit(1);
    }
    if b.v.is_none() {
        std::process::exit(2);
    }

    if a.v == b.v {
        std::process::exit(3);
    }

    fill_vec(a.v.unwrap(), a.n, 1.0f32);
    fill_vec(b.v.unwrap(), b.n, 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(a.v.unwrap(), a.n), expected_a) {
        std::process::exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(b.v.unwrap(), b.n), expected_b) {
        std::process::exit(5);
    }

    let h = GLOBAL.lock().unwrap().h;
    if (h & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}