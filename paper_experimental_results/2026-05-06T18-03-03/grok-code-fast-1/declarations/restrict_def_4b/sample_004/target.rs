use std::mem;
use std::process;

static mut POOL: [u8; 8192] = [0; 8192];
static mut OFF: usize = 0;
static mut H: u64 = 0x9e3779b97f4a7c15u64;

fn alloc_bytes(n: usize) -> Option<&'static mut [u8]> {
    if n == 0 {
        return Some(&mut POOL[OFF..OFF]);
    }
    if OFF + n > 8192 {
        return None;
    }
    let start = OFF;
    OFF += n;
    let p = &mut POOL[start..OFF];
    unsafe {
        let addr = p.as_ptr() as usize;
        H ^= (addr as u64).wrapping_add(0x9e3779b97f4a7c15u64);
        H = H.wrapping_mul(0x5851f42d4c957f2d);
    }
    Some(p)
}

struct Vector {
    n: i32,
    v: Option<&'static mut [f32]>,
}

fn new_vector(n: i32) -> Vector {
    let bytes = (n as usize) * mem::size_of::<f32>();
    if let Some(slice) = alloc_bytes(bytes) {
        let v = unsafe {
            std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut f32, n as usize)
        };
        Vector { n, v: Some(v) }
    } else {
        Vector { n, v: None }
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
    let d = x - y;
    d.abs() < 0.0001
}

fn main() {
    let mut a = new_vector(16);
    let mut b = new_vector(16);
    if a.v.is_none() {
        process::exit(1);
    }
    if b.v.is_none() {
        process::exit(2);
    }
    if std::ptr::eq(a.v.as_ref().unwrap().as_ptr(), b.v.as_ref().unwrap().as_ptr()) {
        process::exit(3);
    }
    fill_vec(a.v.as_mut().unwrap(), 1.0);
    fill_vec(b.v.as_mut().unwrap(), 100.0);
    let sum_a = sum_vec(a.v.as_ref().unwrap());
    let expected_a = 1.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_a, expected_a) {
        process::exit(4);
    }
    let sum_b = sum_vec(b.v.as_ref().unwrap());
    let expected_b = 100.0 * 16.0 + (15.0 * 16.0) / 2.0;
    if !close_enough(sum_b, expected_b) {
        process::exit(5);
    }
    if (H & 1) == 0 {
        process::exit(6);
    }
    process::exit(0);
}