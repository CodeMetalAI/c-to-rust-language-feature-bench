use std::sync::{Mutex, OnceLock};

type USize = usize;

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    offset: Option<usize>,
}

const POOL_BYTES: usize = 8192;
const H_INIT: u64 = 0x9e3779b97f4a7c15;
const C1: u64 = 0x9e3779b97f4a7c15;
const C2: u64 = 0x5851f42d4c957f2d;

struct PoolState {
    data: Vec<f32>,
    off_bytes: usize,
    h: u64,
}

static POOL: OnceLock<Mutex<PoolState>> = OnceLock::new();

fn pool() -> &'static Mutex<PoolState> {
    POOL.get_or_init(|| {
        Mutex::new(PoolState {
            data: vec![0.0f32; POOL_BYTES / 4],
            off_bytes: 0,
            h: H_INIT,
        })
    })
}

fn alloc_bytes(n: USize) -> Option<USize> {
    let mut state = pool().lock().unwrap();
    if n == 0 {
        return Some(state.off_bytes);
    }
    if state.off_bytes + n > POOL_BYTES {
        return None;
    }
    let p = state.off_bytes;
    state.off_bytes += n;

    let p_u = p as u64;
    state.h ^= p_u.wrapping_add(C1);
    state.h = state.h.wrapping_mul(C2);

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let bytes = n as usize * std::mem::size_of::<f32>();
    let offset_bytes = alloc_bytes(bytes);
    let offset = offset_bytes.map(|b| b / 4);
    Vector { n, offset }
}

fn fill_vec(idx: usize, n: i32, base: f32) {
    let mut state = pool().lock().unwrap();
    let n_usize = n as usize;
    for i in 0..n_usize {
        state.data[idx + i] = base + i as f32;
    }
}

fn sum_vec(idx: usize, n: i32) -> f32 {
    let state = pool().lock().unwrap();
    let n_usize = n as usize;
    let mut s = 0.0f32;
    for i in 0..n_usize {
        s += state.data[idx + i];
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0f32 {
        d = -d;
    }
    d < 0.0001f32
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.offset.is_none() {
        std::process::exit(1);
    }
    if b.offset.is_none() {
        std::process::exit(2);
    }

    if a.offset == b.offset {
        std::process::exit(3);
    }

    let a_idx = a.offset.unwrap();
    let b_idx = b.offset.unwrap();

    fill_vec(a_idx, a.n, 1.0f32);
    fill_vec(b_idx, b.n, 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(a_idx, a.n), expected_a) {
        std::process::exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(b_idx, b.n), expected_b) {
        std::process::exit(5);
    }

    let h = pool().lock().unwrap().h;
    if (h & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}