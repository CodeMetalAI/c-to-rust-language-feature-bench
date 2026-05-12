use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    offset: Option<usize>,
}

struct State {
    pool: Vec<f32>,
    off: usize, // in bytes
    h: u64,
}

const POOL_BYTES: usize = 8192;
const FLOAT_SIZE: usize = 4;
const CONST1: u64 = 0x9e3779b97f4a7c15u64;
const CONST2: u64 = 0x5851f42d4c957f2du64;

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(State {
            pool: vec![0.0f32; POOL_BYTES / FLOAT_SIZE],
            off: 0,
            h: CONST1,
        })
    })
}

fn alloc_bytes(n: usize) -> Option<usize> {
    let mut st = state().lock().unwrap();
    if n == 0 {
        return Some(st.off);
    }
    if st.off + n > POOL_BYTES {
        return None;
    }
    let p = st.off;
    st.off += n;

    let p_u64 = p as u64;
    st.h = st.h ^ p_u64.wrapping_add(CONST1);
    st.h = st.h.wrapping_mul(CONST2);

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let bytes = n as usize * FLOAT_SIZE;
    let offset = alloc_bytes(bytes);
    Vector { n, offset }
}

fn fill_vec(offset: usize, n: i32, base: f32) {
    let mut st = state().lock().unwrap();
    let start = offset / FLOAT_SIZE;
    let count = n as usize;
    for i in 0..count {
        st.pool[start + i] = base + i as f32;
    }
}

fn sum_vec(offset: usize, n: i32) -> f32 {
    let st = state().lock().unwrap();
    let start = offset / FLOAT_SIZE;
    let count = n as usize;
    let mut s = 0.0f32;
    for i in 0..count {
        s += st.pool[start + i];
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

    if a.offset.is_none() {
        std::process::exit(1);
    }
    if b.offset.is_none() {
        std::process::exit(2);
    }

    if a.offset == b.offset {
        std::process::exit(3);
    }

    let a_off = a.offset.unwrap();
    let b_off = b.offset.unwrap();

    fill_vec(a_off, a.n, 1.0f32);
    fill_vec(b_off, b.n, 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(a_off, a.n), expected_a) {
        std::process::exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(b_off, b.n), expected_b) {
        std::process::exit(5);
    }

    let h = state().lock().unwrap().h;
    if (h & 1u64) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}