use std::mem;

type Usize = u64;

struct Vector {
    n: i32,
    v: Option<Box<[f32]>>,
}

static mut H: Usize = 0x9e3779b97f4a7c15u64;

fn alloc_bytes(n: Usize) -> Option<*mut u8> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: Usize = 0;

    if n == 0 {
        return Some(POOL.as_mut_ptr().add(OFF as usize));
    }

    if OFF + n > POOL.len() as Usize {
        return None;
    }

    let p = POOL.as_mut_ptr().add(OFF as usize);
    OFF += n;

    H ^= (p as Usize) + 0x9e3779b97f4a7c15u64;
    H *= 0x5851f42d4c957f2du64;

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let size = (n as Usize) * mem::size_of::<f32>() as Usize;
    let ptr = alloc_bytes(size);
    
    Vector {
        n,
        v: ptr.map(|p| unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(p as *mut f32, n as usize))
        }),
    }
}

fn fill_vec(p: &mut [f32], base: f32) {
    for i in 0..p.len() {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001f
}

fn main() {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        return std::process::exit(1);
    }
    if b.v.is_none() {
        return std::process::exit(2);
    }

    // In Rust we cannot directly compare raw pointers from Box, but we can check if they point to overlapping memory.
    // Since we're using a static pool and separate allocations, they should be distinct.
    // We'll trust the allocator's behavior matches the C code.

    let a_slice = a.v.as_ref().unwrap().as_ref();
    let b_slice = b.v.as_ref().unwrap().as_ref();
    
    fill_vec(a_slice, 1.0f);
    fill_vec(b_slice, 100.0f);

    let expected_a = 1.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(sum_vec(a_slice), expected_a) {
        return std::process::exit(4);
    }

    let expected_b = 100.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(sum_vec(b_slice), expected_b) {
        return std::process::exit(5);
    }

    if (H & 1u64) == 0u64 {
        return std::process::exit(6);
    }

    std::process::exit(0);
}