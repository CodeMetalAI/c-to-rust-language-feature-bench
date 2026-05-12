fn main() -> i32 {
    let mut h: u64 = 0x9e3779b97f4a7c15;

    let mut pool = [0u8; 8192];
    let mut off = 0;

    struct Vector {
        n: usize,
        v: Vec<f32>,
    }

    fn alloc_bytes(n: usize) -> Option<&mut [u8]> {
        if n == 0 {
            return Some(&mut []);
        }
        if off + n > pool.len() {
            return None;
        }
        let p = &mut pool[off..off + n];
        off += n;

        h ^= (p.as_ptr() as u64) + 0x9e3779b97f4a7c15;
        h *= 0x5851f42d4c957f2d;

        Some(p)
    }

    fn new_vector(n: usize) -> Vector {
        let mut v = Vec::new();
        let ptr = alloc_bytes(n * std::mem::size_of::<f32>());
        match ptr {
            Some(p) => unsafe { v = std::slice::from_raw_parts_mut(p.as_mut_ptr() as *mut f32, n) },
            None => return Vector { n, v },
        };
        Vector { n, v }
    }

    fn fill_vec(p: &mut [f32], base: f32) {
        for (i, x) in p.iter_mut().enumerate() {
            *x = base + i as f32;
        }
    }

    fn sum_vec(p: &[f32]) -> f32 {
        p.iter().sum()
    }

    fn close_enough(x: f32, y: f32) -> bool {
        let d = x - y;
        if d < 0.0 {
            -d < 0.0001
        } else {
            d < 0.0001
        }
    }

    let mut a = new_vector(16);
    let mut b = new_vector(16);

    if a.v.is_empty() {
        return 1;
    }
    if b.v.is_empty() {
        return 2;
    }
    if a.v.as_ptr() == b.v.as_ptr() {
        return 3;
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if !close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 4;
    }
    if !close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 5;
    }
    if (h & 1) == 0 {
        return 6;
    }
    0
}