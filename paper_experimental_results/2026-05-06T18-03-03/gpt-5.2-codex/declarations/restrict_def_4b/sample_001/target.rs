use std::cell::{Cell, RefCell};
use std::process::exit;

const POOL_SIZE: usize = 8192;
const BASE_PTR: u64 = 0x1000_0000;
const H_INIT: u64 = 0x9e3779b97f4a7c15;
const H_XOR_CONST: u64 = 0x9e3779b97f4a7c15;
const H_MUL_CONST: u64 = 0x5851f42d4c957f2d;

#[derive(Copy, Clone)]
struct Vector {
    n: i32,
    v: u64,
}

struct Alloc {
    pool: [u8; POOL_SIZE],
    off: usize,
}

thread_local! {
    static ALLOC: RefCell<Alloc> = RefCell::new(Alloc { pool: [0u8; POOL_SIZE], off: 0 });
    static H: Cell<u64> = Cell::new(H_INIT);
}

fn alloc_bytes(n: usize) -> u64 {
    ALLOC.with(|alloc_cell| {
        let mut alloc = alloc_cell.borrow_mut();
        if n == 0 {
            return BASE_PTR.wrapping_add(alloc.off as u64);
        }
        if alloc.off + n > POOL_SIZE {
            return 0;
        }
        let p = BASE_PTR.wrapping_add(alloc.off as u64);
        alloc.off += n;

        H.with(|hcell| {
            let mut h = hcell.get();
            h = h ^ p.wrapping_add(H_XOR_CONST);
            h = h.wrapping_mul(H_MUL_CONST);
            hcell.set(h);
        });

        p
    })
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * std::mem::size_of::<f32>();
    let v = alloc_bytes(bytes);
    Vector { n, v }
}

fn write_f32(offset: usize, val: f32) {
    ALLOC.with(|alloc_cell| {
        let mut alloc = alloc_cell.borrow_mut();
        let bytes = val.to_le_bytes();
        alloc.pool[offset..offset + 4].copy_from_slice(&bytes);
    });
}

fn read_f32(offset: usize) -> f32 {
    ALLOC.with(|alloc_cell| {
        let alloc = alloc_cell.borrow();
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&alloc.pool[offset..offset + 4]);
        f32::from_le_bytes(bytes)
    })
}

fn fill_vec(p: u64, n: i32, base: f32) {
    let base_off = (p - BASE_PTR) as usize;
    let mut i = 0;
    while i < n {
        let off = base_off + (i as usize) * 4;
        write_f32(off, base + i as f32);
        i += 1;
    }
}

fn sum_vec(p: u64, n: i32) -> f32 {
    let base_off = (p - BASE_PTR) as usize;
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        let off = base_off + (i as usize) * 4;
        s += read_f32(off);
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

    if a.v == 0 {
        exit(1);
    }
    if b.v == 0 {
        exit(2);
    }

    if a.v == b.v {
        exit(3);
    }

    fill_vec(a.v, a.n, 1.0);
    fill_vec(b.v, b.n, 100.0);

    let expected_a = 1.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_vec(a.v, a.n), expected_a) {
        exit(4);
    }

    let expected_b = 100.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_vec(b.v, b.n), expected_b) {
        exit(5);
    }

    let h = H.with(|hcell| hcell.get());
    if (h & 1) == 0 {
        exit(6);
    }

    exit(0);
}