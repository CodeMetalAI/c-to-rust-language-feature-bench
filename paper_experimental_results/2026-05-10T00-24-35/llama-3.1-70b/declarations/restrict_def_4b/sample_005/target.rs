fn main() {
    let mut h: u64 = 0x9e3779b97f4a7c15;

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

        let p = &mut POOL[OFF..];
        OFF += n;

        h ^= (p.as_ptr() as u64) ^ 0x9e3779b97f4a7c15;
        h *= 0x5851f42d4c957f2d;

        Some(p.to_vec())
    }

    fn new_vector(n: usize) -> Option<Vector> {
        alloc_bytes(n * std::mem::size_of::<f32>())
           .map(|v| Vector { n, v: unsafe { std::mem::transmute(v) } })
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
        (x - y).abs() < 0.0001
    }

    let a = new_vector(16).unwrap();
    let b = new_vector(16).unwrap();

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

    if!close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 4;
    }

    if!close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 5;
    }

    if h % 2 == 0 {
        return 6;
    }

    std::process::exit(0);
}