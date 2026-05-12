fn main() {
    const POOL_SIZE: usize = 8192;
    static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];
    static mut OFFSET: usize = 0;
    static mut H: u64 = 0x9e3779b97f4a7c15;

    struct Vector {
        n: i32,
        v: Vec<f32>,
    }

    fn alloc_bytes(n: usize) -> Option<&'static mut [u8]> {
        if n == 0 {
            return Some(&mut POOL[OFFSET..OFFSET]);
        }
        if OFFSET + n > POOL_SIZE {
            return None;
        }
        let p = &mut POOL[OFFSET..OFFSET + n];
        OFFSET += n;
        unsafe {
            H ^= (OFFSET as u64) ^ 0x9e3779b97f4a7c15;
            H *= 0x5851f42d4c957f2d;
        }
        Some(p)
    }

    fn new_vector(n: i32) -> Vector {
        let v = alloc_bytes(n as usize * std::mem::size_of::<f32>()).unwrap();
        Vector {
            n,
            v: unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut f32, n as usize).to_vec() },
        }
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
    if (unsafe { H } & 1 == 0) {
        std::process::exit(6);
    }
}