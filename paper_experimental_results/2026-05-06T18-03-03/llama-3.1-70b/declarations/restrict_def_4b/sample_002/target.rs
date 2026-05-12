fn main() {
    const H: u64 = 0x9e3779b97f4a7c15;
    const H_MUL: u64 = 0x5851f42d4c957f2d;

    struct Vector {
        n: usize,
        v: Vec<f32>,
    }

    fn alloc_bytes(n: usize) -> Option<Vec<u8>> {
        static mut POOL: [u8; 8192] = [0; 8192];
        static mut OFF: usize = 0;

        if n == 0 {
            return Some(Vec::new());
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = &mut POOL[OFF..OFF + n];
        OFF += n;

        let mut h = H;
        h ^= (OFF as u64) ^ H;
        h *= H_MUL;

        Some(p.to_vec())
    }

    fn new_vector(n: usize) -> Option<Vector> {
        let v = alloc_bytes(n * std::mem::size_of::<f32>())?;
        Some(Vector { n, v: unsafe { std::mem::transmute(v) } })
    }

    fn fill_vec(p: &mut [f32], n: usize, base: f32) {
        for i in 0..n {
            p[i] = base + i as f32;
        }
    }

    fn sum_vec(p: &[f32], n: usize) -> f32 {
        p.iter().take(n).sum()
    }

    fn close_enough(x: f32, y: f32) -> bool {
        let d = x - y;
        if d < 0.0 {
            d = -d;
        }
        d < 0.0001
    }

    let mut a = new_vector(16).unwrap();
    let mut b = new_vector(16).unwrap();

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

    if (H & 1) == 0 {
        return;
    }

    std::process::exit(0);
}