fn main() {
    const H_INIT: u64 = 0x9e3779b97f4a7c15;
    const H_MULT: u64 = 0x5851f42d4c957f2d;

    struct Vector {
        n: usize,
        v: Vec<f32>,
    }

    static mut POOL: [u8; 8192] = [0; 8192];
    static mut POOL_OFF: usize = 0;
    static mut H: u64 = H_INIT;

    unsafe fn alloc_bytes(n: usize) -> *mut u8 {
        if n == 0 {
            POOL_OFF += n;
            return POOL.as_mut_ptr().add(POOL_OFF - n);
        }

        if POOL_OFF + n > POOL.len() {
            return std::ptr::null_mut();
        }

        let p = POOL.as_mut_ptr().add(POOL_OFF);
        POOL_OFF += n;

        H ^= (p as u64) ^ H_INIT;
        H *= H_MULT;

        p
    }

    fn new_vector(n: usize) -> Vector {
        let v = unsafe { std::slice::from_raw_parts_mut(alloc_bytes(n * std::mem::size_of::<f32>()) as *mut f32, n) }.to_vec();
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
        let d = (x - y).abs();
        d < 0.0001
    }

    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_empty() {
        std::process::exit(1);
    }
    if b.v.is_empty() {
        std::process::exit(2);
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        std::process::exit(3);
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if!close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(4);
    }

    if!close_enough(sum_vec(&b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        std::process::exit(5);
    }

    if unsafe { H & 1 } == 0 {
        std::process::exit(6);
    }

    std::process::exit(0);
}