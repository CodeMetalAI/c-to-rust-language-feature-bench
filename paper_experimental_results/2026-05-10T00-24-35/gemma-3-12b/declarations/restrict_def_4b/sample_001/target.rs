#[derive(Debug)]
struct vector {
    n: usize,
    v: Vec<f32>,
}

static mut h: u64 = 0x9e3779b97f4a7c15u64;
static mut pool: [u8; 8192] = [0; 8192];
static mut off: usize = 0;

fn alloc_bytes(n: usize) -> *mut u8 {
    if n == 0 {
        return pool.as_mut_ptr().add(off);
    }

    if off + n > pool.len() {
        return std::ptr::null_mut();
    }

    let p = pool.as_mut_ptr().add(off);
    off += n;

    unsafe {
        h ^= (p as u64) + 0x9e3779b97f4a7c15u64;
        h *= 0x5851f42d4c957f2dull;
    }

    p
}

fn new_vector(n: usize) -> vector {
    let v = unsafe { Vec::from_raw_parts(alloc_bytes(n * std::mem::size_of::<f32>()), 0, n) };
    vector { n, v }
}

fn fill_vec(p: &mut [f32], n: usize, base: f32) {
    for i in 0..n {
        p[i] = base + (i as f32);
    }
}

fn sum_vec(p: &[f32], n: usize) -> f32 {
    let mut s = 0.0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001f32
}

fn main() -> i32 {
    unsafe {
        let a = new_vector(16);
        let b = new_vector(16);

        if a.v.is_null() {
            return 1;
        }
        if b.v.is_null() {
            return 2;
        }

        if std::ptr::eq(a.v, b.v) {
            return 3;
        }

        fill_vec(&mut a.v, a.n, 1.0f32);
        fill_vec(&mut b.v, b.n, 100.0f32);

        if !close_enough(sum_vec(&a.v, a.n), 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            return 4;
        }

        if !close_enough(sum_vec(&b.v, b.n),
                         100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
            return 5;
        }

        if (h & 1) == 0 {
            return 6;
        }

        0
    }
}