use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref OFF: Mutex<usize> = Mutex::new(0);
    static ref H: Mutex<u64> = Mutex::new(0x9e3779b97f4a7c15u64);
    static ref ID_COUNTER: Mutex<usize> = Mutex::new(0);
}

struct Vector {
    n: i32,
    v: Option<Vec<f32>>,
    id: usize,
}

fn new_vector(n: i32) -> Vector {
    let size = n as usize * 4;
    let mut off = OFF.lock().unwrap();
    if *off + size > 8192 {
        return Vector { n, v: None, id: 0 };
    }
    *off += size;
    let mut h = H.lock().unwrap();
    *h ^= (*off as u64) + 0x9e3779b97f4a7c15u64;
    *h = (*h).wrapping_mul(0x5851f42d4c957f2du64);
    let mut id_counter = ID_COUNTER.lock().unwrap();
    let id = *id_counter;
    *id_counter += 1;
    Vector { n, v: Some(vec![0.0; n as usize]), id }
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n as usize {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    p.iter().take(n as usize).sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    d.abs() < 0.0001
}

fn main() -> i32 {
    let a = new_vector(16);
    if a.v.is_none() {
        return 1;
    }
    let b = new_vector(16);
    if b.v.is_none() {
        return 2;
    }
    if a.id == b.id {
        return 3;
    }
    fill_vec(a.v.as_mut().unwrap(), a.n, 1.0);
    fill_vec(b.v.as_mut().unwrap(), b.n, 100.0);
    if !close_enough(sum_vec(a.v.as_ref().unwrap(), a.n), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 4;
    }
    if !close_enough(sum_vec(b.v.as_ref().unwrap(), b.n), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 5;
    }
    if (*H.lock().unwrap() & 1) == 0 {
        return 6;
    }
    0
}