use std::mem;

type Usize = u64;

struct Vector {
    n: i32,
    v: Option<Box<[f32]>>,
}

static mut H: Usize = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: Usize) -> Option<*mut u8> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: Usize = 0;

    if n == 0 {
        return Some((POOL.as_mut_ptr() as *mut u8).add(OFF as usize));
    }

    if OFF + n > POOL.len() as Usize {
        return None;
    }

    let p = (POOL.as_mut_ptr() as *mut u8).add(OFF as usize);
    OFF += n;

    H ^= p as Usize + 0x9e3779b97f4a7c15;
    H *= 0x5851f42d4c957f2d;

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let bytes_needed = (n as Usize) * mem::size_of::<f32>() as Usize;
    let ptr = alloc_bytes(bytes_needed);
    
    let v = if let Some(raw_ptr) = ptr {
        let slice_ptr = raw_ptr as *mut f32;
        let slice = unsafe { std::slice::from_raw_parts_mut(slice_ptr, n as usize) };
        Some(Box::from_raw(slice))
    } else {
        None
    };
    
    Vector { n, v }
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
    (x - y).abs() < 0.0001f
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

    // Since we're using Box, the addresses are unique and won't be equal
    // but we check if they point to the same memory region by comparing pointers
    let a_ptr = a.v.as_ref().unwrap().as_ptr();
    let b_ptr = b.v.as_ref().unwrap().as_ptr();
    if a_ptr == b_ptr {
        return std::process::exit(3);
    }

    fill_vec(a.v.as_mut().unwrap().as_mut(), 1.0f);
    fill_vec(b.v.as_mut().unwrap().as_mut(), 100.0f);

    let a_sum = sum_vec(a.v.as_ref().unwrap().as_ref());
    let expected_a = 1.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(a_sum, expected_a) {
        return std::process::exit(4);
    }

    let b_sum = sum_vec(b.v.as_ref().unwrap().as_ref());
    let expected_b = 100.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(b_sum, expected_b) {
        return std::process::exit(5);
    }

    if (H & 1) == 0 {
        return std::process::exit(6);
    }

    std::process::exit(0);
}