use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::process::exit;

type Usize = usize;

#[derive(Clone, Copy)]
struct Vector {
    n: i32,
    v: Option<Usize>,
}

struct Pool {
    data: [u8; 8192],
    off: Usize,
}

thread_local! {
    static POOL: RefCell<Pool> = RefCell::new(Pool { data: [0u8; 8192], off: 0 });
    static H: Cell<Usize> = Cell::new(0x9e3779b97f4a7c15usize);
}

fn alloc_bytes(n: Usize) -> Option<Usize> {
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();

        if n == 0 {
            return Some(pool.off);
        }

        if pool.off + n > pool.data.len() {
            return None;
        }

        let p_offset = pool.off;
        pool.off += n;

        let base_addr = pool.data.as_ptr() as Usize;
        let p_val = base_addr.wrapping_add(p_offset);

        H.with(|hcell| {
            let mut h = hcell.get();
            h = h ^ p_val.wrapping_add(0x9e3779b97f4a7c15usize);
            h = h.wrapping_mul(0x5851f42d4c957f2dusize);
            hcell.set(h);
        });

        Some(p_offset)
    })
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as Usize) * size_of::<f32>();
    Vector { n, v: alloc_bytes(bytes) }
}

fn fill_vec(p_offset: Usize, n: i32, base: f32) {
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();
        let mut i = 0;
        while i < n {
            let val = base + i as f32;
            let bytes = val.to_ne_bytes();
            let idx = p_offset + (i as Usize) * 4;
            pool.data[idx..idx + 4].copy_from_slice(&bytes);
            i += 1;
        }
    });
}

fn sum_vec(p_offset: Usize, n: i32) -> f32 {
    POOL.with(|pool_cell| {
        let pool = pool_cell.borrow();
        let mut s = 0.0f32;
        let mut i = 0;
        while i < n {
            let idx = p_offset + (i as Usize) * 4;
            let bytes = [
                pool.data[idx],
                pool.data[idx + 1],
                pool.data[idx + 2],
                pool.data[idx + 3],
            ];
            s += f32::from_ne_bytes(bytes);
            i += 1;
        }
        s
    })
}

fn close_enough(x: f32, y: f32) -> bool {
    (x - y).abs() < 0.0001f32
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

    let expected_a = 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(a.v.unwrap(), a.n), expected_a) {
        exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32;
    if !close_enough(sum_vec(b.v.unwrap(), b.n), expected_b) {
        exit(5);
    }

    let h_val = H.with(|hcell| hcell.get());
    if (h_val & 1usize) == 0usize {
        exit(6);
    }

    exit(0);
}