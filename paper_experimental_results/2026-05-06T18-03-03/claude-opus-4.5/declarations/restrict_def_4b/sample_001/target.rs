struct Vector {
    n: i32,
    v: Option<usize>, // Index into the pool
}

struct Allocator {
    pool: [u8; 8192],
    off: usize,
    h: u64,
}

impl Allocator {
    fn new() -> Self {
        Allocator {
            pool: [0u8; 8192],
            off: 0,
            h: 0x9e3779b97f4a7c15u64,
        }
    }

    fn alloc_bytes(&mut self, n: usize) -> Option<usize> {
        if n == 0 {
            return Some(self.off);
        }

        if self.off + n > self.pool.len() {
            return None;
        }

        let p = self.off;
        self.off += n;

        // Simulate pointer arithmetic for hash
        self.h ^= (p as u64).wrapping_add(0x9e3779b97f4a7c15u64);
        self.h = self.h.wrapping_mul(0x5851f42d4c957f2du64);

        Some(p)
    }

    fn new_vector(&mut self, n: i32) -> Vector {
        let size = (n as usize) * std::mem::size_of::<f32>();
        let v = self.alloc_bytes(size);
        Vector { n, v }
    }

    fn get_float_slice_mut(&mut self, start: usize, count: i32) -> &mut [f32] {
        let byte_len = (count as usize) * std::mem::size_of::<f32>();
        let bytes = &mut self.pool[start..start + byte_len];
        let ptr = bytes.as_mut_ptr() as *mut f32;
        unsafe { std::slice::from_raw_parts_mut(ptr, count as usize) }
    }

    fn get_float_slice(&self, start: usize, count: i32) -> &[f32] {
        let byte_len = (count as usize) * std::mem::size_of::<f32>();
        let bytes = &self.pool[start..start + byte_len];
        let ptr = bytes.as_ptr() as *const f32;
        unsafe { std::slice::from_raw_parts(ptr, count as usize) }
    }
}

fn fill_vec(p: &mut [f32], base: f32) {
    let mut i = 0;
    while i < p.len() {
        p[i] = base + (i as f32);
        i += 1;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    let mut s = 0.0f32;
    let mut i = 0;
    while i < p.len() {
        s += p[i];
        i += 1;
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
    let mut allocator = Allocator::new();

    let a = allocator.new_vector(16);
    let b = allocator.new_vector(16);

    let a_v = match a.v {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let b_v = match b.v {
        Some(v) => v,
        None => std::process::exit(2),
    };

    if a_v == b_v {
        std::process::exit(3);
    }

    // Fill vector a
    {
        let slice = allocator.get_float_slice_mut(a_v, a.n);
        fill_vec(slice, 1.0f32);
    }

    // Fill vector b
    {
        let slice = allocator.get_float_slice_mut(b_v, b.n);
        fill_vec(slice, 100.0f32);
    }

    // Check sum of a
    {
        let slice = allocator.get_float_slice(a_v, a.n);
        if !close_enough(sum_vec(slice), 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            std::process::exit(4);
        }
    }

    // Check sum of b
    {
        let slice = allocator.get_float_slice(b_v, b.n);
        if !close_enough(sum_vec(slice), 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            std::process::exit(5);
        }
    }

    if (allocator.h & 1u64) == 0u64 {
        std::process::exit(6);
    }

    std::process::exit(0);
}