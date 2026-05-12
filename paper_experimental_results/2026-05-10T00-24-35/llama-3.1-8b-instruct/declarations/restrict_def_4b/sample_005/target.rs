fn alloc_bytes(n: usize) -> *mut u8 {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: usize = 0;

    if n == 0 {
        return POOL.as_mut_ptr().offset(OFF as isize);
    }

    if OFF + n > POOL.len() {
        return std::ptr::null_mut();
    }

    let p = POOL.as_mut_ptr().offset(OFF as isize);
    OFF += n;

    let mut h = 0x9e3779b97f4a7c15u64;
    h ^= (p as u64) + 0x9e3779b97f4a7c15u64;
    h *= 0x5851f42d4c957f2du64;

    p
}

struct Vector {
    n: i32,
    v: *mut f32,
}

impl Vector {
    fn new(n: i32) -> Self {
        let v = unsafe { alloc_bytes(n as usize * std::mem::size_of::<f32>()) as *mut f32 };
        Self { n, v }
    }
}

fn fill_vec(v: &mut [f32], base: f32) {
    for i in 0..v.len() {
        v[i] = base + i as f32;
    }
}

fn sum_vec(v: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in 0..v.len() {
        sum += v[i];
    }
    sum
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001
}

fn main() {
    let a = Vector::new(16);
    let b = Vector::new(16);

    if a.v.is_null() {
        eprintln!("Error: out of memory");
        std::process::exit(1);
    }
    if b.v.is_null() {
        eprintln!("Error: out of memory");
        std::process::exit(2);
    }

    if a.v == b.v {
        eprintln!("Error: a.v and b.v are the same");
        std::process::exit(3);
    }

    fill_vec(a.v, 1.0);
    fill_vec(b.v, 100.0);

    if!close_enough(sum_vec(a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        eprintln!("Error: sum of a.v is incorrect");
        std::process::exit(4);
    }

    if!close_enough(sum_vec(b.v), 100.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        eprintln!("Error: sum of b.v is incorrect");
        std::process::exit(5);
    }

    if (0x9e3779b97f4a7c15u64 & 1u64) == 0u64 {
        eprintln!("Error: h is even");
        std::process::exit(6);
    }
}