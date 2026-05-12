use std::mem;

type Usize = u64;

struct Vector {
    n: i32,
    v: Option<Box<[f32]>>,
}

static mut H: Usize = 0x9e3779b97f4a7c15u64;
static mut POOL: [u8; 8192] = [0; 8192];
static mut OFF: Usize = 0;

fn alloc_bytes(n: Usize) -> Option<&'static mut [u8]> {
    unsafe {
        if n == 0 {
            return Some(&mut POOL[OFF as usize..OFF as usize]);
        }

        if OFF + n > POOL.len() as Usize {
            return None;
        }

        let start = OFF as usize;
        OFF += n;
        let slice = &mut POOL[start..start + n as usize];

        H ^= slice.as_ptr() as Usize + 0x9e3779b97f4a7c15u64;
        H = H.wrapping_mul(0x5851f42d4c957f2du64);

        Some(slice)
    }
}

fn new_vector(n: i32) -> Vector {
    let total_bytes = (n as Usize).wrapping_mul(mem::size_of::<f32>() as Usize);
    let slice_opt = alloc_bytes(total_bytes);
    
    let v = slice_opt.map(|bytes| {
        let ptr = bytes.as_ptr() as *mut f32;
        let len = bytes.len() / mem::size_of::<f32>();
        unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)) }
    });

    Vector { n, v }
}

fn fill_vec(p: &mut [f32], base: f32) {
    for (i, elem) in p.iter_mut().enumerate() {
        *elem = base + i as f32;
    }
}

fn sum_vec(p: &[f32]) -> f32 {
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    (x - y).abs() < 0.0001f32
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

    // Check if they point to the same memory (should not happen with our allocator)
    let a_ptr = a.v.as_ref().unwrap().as_ptr();
    let b_ptr = b.v.as_ref().unwrap().as_ptr();
    if a_ptr == b_ptr {
        std::process::exit(3);
    }

    fill_vec(a.v.as_mut().unwrap(), 1.0f32);
    fill_vec(b.v.as_mut().unwrap(), 100.0f32);

    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(a.v.as_ref().unwrap()), expected_a) {
        std::process::exit(4);
    }

    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(b.v.as_ref().unwrap()), expected_b) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}