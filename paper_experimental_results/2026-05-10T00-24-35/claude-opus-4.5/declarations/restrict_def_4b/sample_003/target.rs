struct Vector {
    n: i32,
    v: Option<usize>, // index into pool
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
        let ptr_val = p as u64;
        self.h ^= ptr_val.wrapping_add(0x9e3779b97f4a7c15u64);
        self.h = self.h.wrapping_mul(0x5851f42d4c957f2du64);

        Some(p)
    }

    fn new_vector(&mut self, n: i32) -> Vector {
        let size = (n as usize) * std::mem::size_of::<f32>();
        let v = self.alloc_bytes(size);
        Vector { n, v }
    }

    fn get_float(&self, base: usize, index: usize) -> f32 {
        let offset = base + index * std::mem::size_of::<f32>();
        let bytes: [u8; 4] = [
            self.pool[offset],
            self.pool[offset + 1],
            self.pool[offset + 2],
            self.pool[offset + 3],
        ];
        f32::from_ne_bytes(bytes)
    }

    fn set_float(&mut self, base: usize, index: usize, value: f32) {
        let offset = base + index * std::mem::size_of::<f32>();
        let bytes = value.to_ne_bytes();
        self.pool[offset] = bytes[0];
        self.pool[offset + 1] = bytes[1];
        self.pool[offset + 2] = bytes[2];
        self.pool[offset + 3] = bytes[3];
    }

    fn fill_vec(&mut self, p: usize, n: i32, base: f32) {
        let mut i = 0;
        while i < n {
            self.set_float(p, i as usize, base + (i as f32));
            i += 1;
        }
    }

    fn sum_vec(&self, p: usize, n: i32) -> f32 {
        let mut s = 0.0f32;
        let mut i = 0;
        while i < n {
            s += self.get_float(p, i as usize);
            i += 1;
        }
        s
    }
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

    if a.v.is_none() {
        std::process::exit(1);
    }
    if b.v.is_none() {
        std::process::exit(2);
    }

    let a_v = a.v.unwrap();
    let b_v = b.v.unwrap();

    if a_v == b_v {
        std::process::exit(3);
    }

    allocator.fill_vec(a_v, a.n, 1.0f32);
    allocator.fill_vec(b_v, b.n, 100.0f32);

    if !close_enough(
        allocator.sum_vec(a_v, a.n),
        1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32,
    ) {
        std::process::exit(4);
    }

    if !close_enough(
        allocator.sum_vec(b_v, b.n),
        100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32,
    ) {
        std::process::exit(5);
    }

    if (allocator.h & 1u64) == 0u64 {
        std::process::exit(6);
    }

    std::process::exit(0);
}