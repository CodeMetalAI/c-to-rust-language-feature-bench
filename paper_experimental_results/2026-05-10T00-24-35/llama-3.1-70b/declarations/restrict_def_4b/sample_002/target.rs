fn main() {
    const H_INIT: u64 = 0x9e3779b97f4a7c15;
    const POOL_SIZE: usize = 8192;
    const HASH_MULT: u64 = 0x5851f42d4c957f2d;

    let mut h: u64 = H_INIT;
    let mut pool: [u8; POOL_SIZE] = [0; POOL_SIZE];
    let mut off: usize = 0;

    struct Vector {
        n: i32,
        v: Vec<f32>,
    }

    fn alloc_bytes(n: usize) -> Option<&'static mut [u8]> {
        if n == 0 {
            return Some(&mut pool[off..]);
        }
        if off + n > POOL_SIZE {
            return None;
        }
        let p = &mut pool[off..off + n];
        off += n;
        h ^= (p.as_ptr() as u64) ^ H_INIT;
        h *= HASH_MULT;
        Some(p)
    }

    fn new_vector(n: i32) -> Vector {
        Vector {
            n,
            v: alloc_bytes((n as usize) * std::mem::size_of::<f32>())
               .unwrap()
               .chunks_mut(std::mem::size_of::<f32>())
               .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
               .collect(),
        }
    }

    fn fill_vec(p: &mut [f32], n: i32, base: f32) {
        for i in 0..n {
            p[i as usize] = base + (i as f32);
        }
    }

    fn sum_vec(p: &[f32], n: i32) -> f32 {
        p.iter().take(n as usize).sum()
    }

    fn close_enough(x: f32, y: f32) -> bool {
        let d = (x - y).abs();
        d < 0.0001
    }

    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_empty() {
        return;
    }
    if b.v.is_empty() {
        return;
    }
    if a.v.as_ptr() == b.v.as_ptr() {
        return;
    }

    fill_vec(&mut a.v, a.n, 1.0);
    fill_vec(&mut b.v, b.n, 100.0);

    if!close_enough(sum_vec(&a.v, a.n), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return;
    }
    if!close_enough(sum_vec(&b.v, b.n), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return;
    }
    if h & 1 == 0 {
        return;
    }
}