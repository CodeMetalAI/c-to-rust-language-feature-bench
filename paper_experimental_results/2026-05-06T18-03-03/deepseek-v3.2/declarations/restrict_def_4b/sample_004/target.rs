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
        return Some((&mut POOL[OFF]) as *mut u8);
    }

    if OFF + n > POOL.len() as Usize {
        return None;
    }

    let p = (&mut POOL[OFF]) as *mut u8;
    OFF += n;

    H ^= (p as Usize) + 0x9e3779b97f4a7c15u64;
    H *= 0x5851f42d4c957f2du64;

    Some(p)
}

fn new_vector(n: i32) -> Vector {
    let size = (n as Usize) * mem::size_of::<f32>() as Usize;
    let ptr = alloc_bytes(size);
    let v = if ptr.is_none() {
        None
    } else {
        let raw_ptr = ptr.unwrap() as *mut f32;
        let slice = unsafe { std::slice::from_raw_parts_mut(raw_ptr, n as usize) };
        Some(Box::from_raw(slice))
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
    let d = (x - y).abs();
    d < 0.0001f
}

fn main() -> i32 {
    let a = new_vector(16);
    let b = new_vector(16);

    if a.v.is_none() {
        return 1;
    }
    if b.v.is_none() {
        return 2;
    }

    // Compare the underlying pointers (Box's unique pointer)
    let a_ptr = a.v.as_ref().unwrap().as_ptr();
    let b_ptr = b.v.as_ref().unwrap().as_ptr();
    if a_ptr == b_ptr {
        return 3;
    }

    fill_vec(a.v.as_mut().unwrap(), 1.0f);
    fill_vec(b.v.as_mut().unwrap(), 100.0f);

    let sum_a = sum_vec(a.v.as_ref().unwrap());
    let expected_a = 1.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(sum_a, expected_a) {
        return 4;
    }

    let sum_b = sum_vec(b.v.as_ref().unwrap());
    let expected_b = 100.0f * 16.0f + (15.0f * 16.0f) / 2.0f;
    if !close_enough(sum_b, expected_b) {
        return 5;
    }

    if (H & 1u64) == 0u64 {
        return 6;
    }

    0
}