fn main() {
    const H_INIT: u64 = 0x9e3779b97f4a7c15;
    const H_MULT: u64 = 0x5851f42d4c957f2d;
    const POOL_SIZE: usize = 8192;

    let mut h = H_INIT;
    let mut pool = [0u8; POOL_SIZE];
    let mut off = 0;

    struct Vector {
        n: i32,
        v: Vec<f32>,
    }

    impl Vector {
        fn new(n: i32) -> Self {
            let v = alloc_bytes(n as usize * std::mem::size_of::<f32>());
            if v.is_null() {
                panic!("allocation failed");
            }
            let v = unsafe { std::slice::from_raw_parts_mut(v as *mut f32, n as usize) };
            Vector { n, v: v.to_vec() }
        }
    }

    fn alloc_bytes(n: usize) -> *mut u8 {
        if n == 0 || off + n > POOL_SIZE {
            return std::ptr::null_mut();
        }
        let p = &mut pool[off] as *mut u8;
        off += n;
        h ^= (p as u64) ^ H_INIT;
        h *= H_MULT;
        p
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
            d = -d;
        }
        d < 0.0001
    }

    let a = Vector::new(16);
    let b = Vector::new(16);

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
    if (h & 1) == 0 {
        std::process::exit(6);
    }
}