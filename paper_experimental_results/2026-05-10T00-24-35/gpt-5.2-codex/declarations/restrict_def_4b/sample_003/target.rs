use std::cell::RefCell;
use std::process::exit;

const INIT_H: u64 = 0x9e3779b97f4a7c15;
const CONST_ADD: u64 = 0x9e3779b97f4a7c15;
const MULT: u64 = 0x5851f42d4c957f2d;
const POOL_BYTES: usize = 8192;
const FLOAT_SIZE: usize = 4;
const POOL_FLOATS: usize = POOL_BYTES / FLOAT_SIZE;

struct State {
    pool: Vec<f32>,
    off: usize,
    h: u64,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        pool: vec![0.0; POOL_FLOATS],
        off: 0,
        h: INIT_H,
    });
}

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: Option<usize>, // offset in bytes
}

fn alloc_bytes(n: usize) -> Option<usize> {
    STATE.with(|s| {
        let mut st = s.borrow_mut();

        if n == 0 {
            return Some(st.off);
        }

        let new_off = st.off.wrapping_add(n);
        if new_off > POOL_BYTES {
            return None;
        }

        let p = st.off;
        st.off = new_off;

        st.h ^= (p as u64).wrapping_add(CONST_ADD);
        st.h = st.h.wrapping_mul(MULT);

        Some(p)
    })
}

fn new_vector(n: i32) -> Vector {
    let n_bytes = (n as usize).wrapping_mul(FLOAT_SIZE);
    let v = alloc_bytes(n_bytes);
    Vector { n, v }
}

fn fill_vec(offset: usize, n: i32, base: f32) {
    let count = n as usize;
    let start = offset / FLOAT_SIZE;
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        for i in 0..count {
            st.pool[start + i] = base + i as f32;
        }
    });
}

fn sum_vec(offset: usize, n: i32) -> f32 {
    let count = n as usize;
    let start = offset / FLOAT_SIZE;
    STATE.with(|s| {
        let st = s.borrow();
        let mut ssum = 0.0f32;
        for i in 0..count {
            ssum += st.pool[start + i];
        }
        ssum
    })
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0 {
        d = -d;
    }
    d < 0.0001
}

fn get_h() -> u64 {
    STATE.with(|s| s.borrow().h)
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