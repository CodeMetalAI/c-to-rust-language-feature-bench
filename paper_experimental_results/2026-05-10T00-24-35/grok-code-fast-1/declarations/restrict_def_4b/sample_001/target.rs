use std::slice;

type Usize = usize;

struct Vector<'a> {
    n: i32,
    v: &'a mut [f32],
}

struct Allocator {
    pool: Vec<u8>,
    off: Usize,
    h: Usize,
}

impl Allocator {
    fn new() -> Self {
        Self {
            pool: vec![0; 8192],
            off: 0,
            h: 0x9e3779b97f4a7c15,
        }
    }

    fn alloc_bytes(&mut self, n: Usize) -> Option<&mut [u8]> {
        if n == 0 {
            return Some(&mut self.pool[self.off..self.off]);
        }
        if self.off + n > self.pool.len() {
            return None;
        }
        let start = self.off;
        self.off += n;
        self.h ^= start + 0x9e3779b97f4a7c15;
        self.h = self.h.wrapping_mul(0x5851f42d4c957f2d);
        Some(&mut self.pool[start..self.off])
    }

    fn alloc_floats(&mut self, n: Usize) -> Option<&mut [f32]> {
        let bytes = self.alloc_bytes(n * std::mem::size_of::<f32>())?;
        if bytes.len() != n * std::mem::size_of::<f32>() {
            return None;
        }
        unsafe { Some(slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut f32, n)) }
    }
}

fn new_vector<'a>(alloc: &'a mut Allocator, n: i32) -> Option<Vector<'a>> {
    Some(Vector {
        n,
        v: alloc.alloc_floats(n as Usize)?,
    })
}

fn fill_vec(p: &mut [f32], base: f32) {
    for (i, val) in p.iter_mut().enumerate() {
        *val = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    d.abs() < 0.0001
}

fn main() {
    let mut alloc = Allocator::new();
    let a_opt = new_vector(&mut alloc, 16);
    if a_opt.is_none() {
        std::process::exit(1);
    }
    let a = a_opt.unwrap();
    let b_opt = new_vector(&mut alloc, 16);
    if b_opt.is_none() {
        std::process::exit(2);
    }
    let b = b_opt.unwrap();
    if a.v.as_ptr() == b.v.as_ptr() {
        std::process::exit(3);
    }
    fill_vec(a.v, 1.0);
    fill_vec(b.v, 100.0);
    let sum_a = sum_vec(a.v);
    let expected_a = 1.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_a, expected_a) {
        std::process::exit(4);
    }
    let sum_b = sum_vec(b.v);
    let expected_b = 100.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_b, expected_b) {
        std::process::exit(5);
    }
    if (alloc.h & 1) == 0 {
        std::process::exit(6);
    }
    std::process::exit(0);
}