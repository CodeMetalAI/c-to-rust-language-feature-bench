struct Vector {
    n: i32,
    v: Option<usize>, // index into pool
}

static mut POOL: [u8; 8192] = [0u8; 8192];
static mut OFF: usize = 0;
static mut H: u64 = 0x9e3779b97f4a7c15u64;

fn alloc_bytes(n: usize) -> Option<usize> {
    unsafe {
        if n == 0 {
            return Some(OFF);
        }

        if OFF + n > POOL.len() {
            return None;
        }

        let p = OFF;
        OFF += n;

        H ^= (p as u64).wrapping_add(0x9e3779b97f4a7c15u64);
        H = H.wrapping_mul(0x5851f42d4c957f2dull);

        Some(p)
    }
}

fn new_vector(n: i32) -> Vector {
    let v = alloc_bytes((n as usize) * std::mem::size_of::<f32>());
    Vector { n, v }
}

fn get_pool_slice(offset: usize, count: usize) -> &'static mut [f32] {
    unsafe {
        let ptr = POOL.as_mut_ptr().add(offset) as *mut f32;
        std::slice::from_raw_parts_mut(ptr, count)
    }
}

fn fill_vec(offset: usize, n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        let slice = get_pool_slice(offset, n as usize);
        slice[i as usize] = base + (i as f32);
        i += 1;
    }
}

fn sum_vec(offset: usize, n: i32) -> f32 {
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        let slice = get_pool_slice(offset, n as usize);
        s += slice[i as usize];
        i += 1;
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let mut d = x - y;
    if d < 0.0f32 {
        d = -d;
    }
    d < 0.0001f32
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        std::process::exit(1);
    }
    if b.v.is_none() {
        std::process::exit(2);
    }

    let a_offset = a.v.unwrap();
    let b_offset = b.v.unwrap();

    if a_offset == b_offset {
        std::process::exit(3);
    }

    fill_vec(a_offset, a.n, 1.0f32);
    fill_vec(b_offset, b.n, 100.0f32);

    if !close_enough(sum_vec(a_offset, a.n), 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(b_offset, b.n), 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}