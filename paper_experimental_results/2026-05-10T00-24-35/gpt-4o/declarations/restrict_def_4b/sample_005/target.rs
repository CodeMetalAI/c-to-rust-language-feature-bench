// restrict_def_4b.rs

type Usize = u64;

#[derive(Debug)]
struct Vector {
    n: i32,
    v: Option<&'static mut [f32]>,
}

static mut H: Usize = 0x9e3779b97f4a7c15;

struct Allocator {
    pool: [u8; 8192],
    off: Usize,
}

impl Allocator {
    const fn new() -> Self {
        Allocator {
            pool: [0; 8192],
            off: 0,
        }
    }

    fn alloc_bytes(&mut self, n: Usize) -> Option<&'static mut [f32]> {
        if n == 0 {
            return Some(&mut []);
        }

        if self.off + n > self.pool.len() as Usize {
            return None;
        }

        let start = self.off as usize;
        let end = (self.off + n) as usize;
        self.off += n;

        unsafe {
            let p = self.pool.as_mut_ptr().add(start) as *mut f32;
            H ^= p as Usize + 0x9e3779b97f4a7c15;
            H = H.wrapping_mul(0x5851f42d4c957f2d);
        }

        Some(unsafe { std::slice::from_raw_parts_mut(self.pool.as_mut_ptr().add(start) as *mut f32, n as usize / std::mem::size_of::<f32>()) })
    }
}

fn new_vector(n: i32, allocator: &mut Allocator) -> Vector {
    let size = (n as Usize) * std::mem::size_of::<f32>() as Usize;
    let v = allocator.alloc_bytes(size);
    Vector { n, v }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n {
        p[i as usize] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    p.iter().take(n as usize).sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() -> i32 {
    let mut allocator = Allocator::new();

    let a = new_vector(16, &mut allocator);
    let b = new_vector(16, &mut allocator);

    if a.v.is_none() {
        return 1;
    }
    if b.v.is_none() {
        return 2;
    }

    if a.v.as_ref().unwrap().as_ptr() == b.v.as_ref().unwrap().as_ptr() {
        return 3;
    }

    fill_vec(a.v.as_mut().unwrap(), a.n, 1.0);
    fill_vec(b.v.as_mut().unwrap(), b.n, 100.0);

    let a_sum = 1.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_vec(a.v.as_ref().unwrap(), a.n), a_sum) {
        return 4;
    }

    let b_sum = 100.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_vec(b.v.as_ref().unwrap(), b.n), b_sum) {
        return 5;
    }

    unsafe {
        if (H & 1) == 0 {
            return 6;
        }
    }

    0
}