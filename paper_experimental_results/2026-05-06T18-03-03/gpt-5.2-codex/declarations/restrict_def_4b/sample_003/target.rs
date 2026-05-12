use std::sync::{Mutex, OnceLock};
use std::process::exit;

type Usize = usize;

const POOL_SIZE: usize = 8192;
const H_INIT: u64 = 0x9e3779b97f4a7c15;
const H_MUL: u64 = 0x5851f42d4c957f2d;

struct Pool {
    buf: [f32; POOL_SIZE / 4],
    off: usize,
}

struct State {
    pool: Pool,
    h: u64,
}

struct Vector {
    n: i32,
    v: Option<usize>,
}

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(State {
            pool: Pool {
                buf: [0.0; POOL_SIZE / 4],
                off: 0,
            },
            h: H_INIT,
        })
    })
}

fn alloc_bytes(n: Usize) -> Option<usize> {
    let mut st = state().lock().unwrap();

    if n == 0 {
        return Some(st.pool.off);
    }

    if st.pool.off + n > POOL_SIZE {
        return None;
    }

    let p_off = st.pool.off;
    st.pool.off += n;

    let base = st.pool.buf.as_ptr() as usize as u64;
    let p_addr = base.wrapping_add(p_off as u64);

    st.h = st.h ^ p_addr.wrapping_add(H_INIT);
    st.h = st.h.wrapping_mul(H_MUL);

    Some(p_off)
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    let v = alloc_bytes(bytes);
    Vector { n, v }
}

fn fill_vec(off: usize, n: i32, base: f32) {
    let mut st = state().lock().unwrap();
    let start = off / 4;
    let mut i = 0;
    while i < n {
        let idx = start + i as usize;
        st.pool.buf[idx] = base + i as f32;
        i += 1;
    }
}

fn sum_vec(off: usize, n: i32) -> f32 {
    let st = state().lock().unwrap();
    let start = off / 4;
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        let idx = start + i as usize;
        s += st.pool.buf[idx];
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

fn get_h() -> u64 {
    state().lock().unwrap().h
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

    if a.v == b.v {
        exit(3);
    }

    fill_vec(a.v.unwrap(), a.n, 1.0f32);
    fill_vec(b.v.unwrap(), b.n, 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(a.v.unwrap(), a.n), expected_a) {
        exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(b.v.unwrap(), b.n), expected_b) {
        exit(5);
    }

    if (get_h() & 1u64) == 0u64 {
        exit(6);
    }

    exit(0);
}