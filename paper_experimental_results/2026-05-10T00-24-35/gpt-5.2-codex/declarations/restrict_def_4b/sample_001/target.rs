use std::cell::{Cell, RefCell};

const POOL_SIZE_BYTES: usize = 8192;
const FLOAT_SIZE: usize = 4;
const POOL_FLOATS: usize = POOL_SIZE_BYTES / FLOAT_SIZE;

const H_INIT: u64 = 0x9e3779b97f4a7c15;
const H_XOR_ADD: u64 = 0x9e3779b97f4a7c15;
const H_MUL: u64 = 0x5851f42d4c957f2d;

struct Pool {
    data: Vec<f32>,
    off_floats: usize,
}

impl Pool {
    fn new() -> Self {
        Self {
            data: vec![0.0f32; POOL_FLOATS],
            off_floats: 0,
        }
    }
}

thread_local! {
    static POOL: RefCell<Pool> = RefCell::new(Pool::new());
    static H: Cell<u64> = Cell::new(H_INIT);
}

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: Option<usize>, // byte offset into pool
}

fn alloc_bytes(n: usize) -> Option<usize> {
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();
        let off_bytes = pool.off_floats * FLOAT_SIZE;

        if n == 0 {
            return Some(off_bytes);
        }

        if off_bytes + n > POOL_SIZE_BYTES {
            return None;
        }

        let p = off_bytes;
        let floats_needed = n / FLOAT_SIZE;
        pool.off_floats += floats_needed;

        H.with(|h_cell| {
            let mut hv = h_cell.get();
            hv ^= (p as u64).wrapping_add(H_XOR_ADD);
            hv = hv.wrapping_mul(H_MUL);
            h_cell.set(hv);
        });

        Some(p)
    })
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * FLOAT_SIZE;
    let v = alloc_bytes(bytes);
    Vector { n, v }
}

fn fill_vec(p: usize, n: i32, base: f32) {
    let start = p / FLOAT_SIZE;
    let count = n as usize;
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();
        for i in 0..count {
            pool.data[start + i] = base + i as f32;
        }
    });
}

fn sum_vec(p: usize, n: i32) -> f32 {
    let start = p / FLOAT_SIZE;
    let count = n as usize;
    POOL.with(|pool_cell| {
        let pool = pool_cell.borrow();
        let mut s = 0.0f32;
        for i in 0..count {
            s += pool.data[start + i];
        }
        s
    })
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

    fill_vec(a.v.unwrap(), a.n, 1.0);
    fill_vec(b.v.unwrap(), b.n, 100.0);

    let expect_a = 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(a.v.unwrap(), a.n), expect_a) {
        std::process::exit(4);
    }

    let expect_b = 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(b.v.unwrap(), b.n), expect_b) {
        std::process::exit(5);
    }

    let h_val = H.with(|h_cell| h_cell.get());
    if (h_val & 1) == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}