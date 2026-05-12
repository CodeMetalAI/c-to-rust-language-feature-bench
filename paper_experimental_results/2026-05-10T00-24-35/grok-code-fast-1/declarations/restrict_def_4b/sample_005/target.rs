use std::sync::Mutex;

type Usize = usize;

static H: Mutex<Usize> = Mutex::new(0x9e3779b97f4a7c15usize);
static OFF: Mutex<Usize> = Mutex::new(0);

#[derive(Clone)]
struct Vector {
    n: i32,
    v: Option<Vec<f32>>,
}

fn alloc_bytes(n: Usize) -> bool {
    let mut off = OFF.lock().unwrap();
    if n == 0 {
        let p = *off as Usize;
        let mut h = H.lock().unwrap();
        *h ^= p.wrapping_add(0x9e3779b97f4a7c15);
        *h = h.wrapping_mul(0x5851f42d4c957f2d);
        return true;
    }
    if *off + n > 8192 {
        return false;
    }
    let p = *off as Usize;
    *off += n;
    let mut h = H.lock().unwrap();
    *h ^= p.wrapping_add(0x9e3779b97f4a7c15);
    *h = h.wrapping_mul(0x5851f42d4c957f2d);
    true
}

fn new_vector(n: i32) -> Vector {
    let success = alloc_bytes((n as usize) * std::mem::size_of::<f32>());
    if success {
        Vector {
            n,
            v: Some(Vec::with_capacity(n as usize)),
        }
    } else {
        Vector { n, v: None }
    }
}

fn fill_vec(v: &mut Vec<f32>, n: i32, base: f32) {
    v.clear();
    for i in 0..n {
        v.push(base + i as f32);
    }
}

fn sum_vec(v: &Vec<f32>) -> f32 {
    v.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    let d = if d < 0.0 { -d } else { d };
    d < 0.0001
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

    if a.v.as_ref() == b.v.as_ref() {
        return 3;
    }

    fill_vec(a.v.as_mut().unwrap(), a.n, 1.0);
    fill_vec(b.v.as_mut().unwrap(), b.n, 100.0);

    if !close_enough(sum_vec(a.v.as_ref().unwrap()), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(b.v.as_ref().unwrap()), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 5;
    }

    let h = *H.lock().unwrap();
    if (h & 1) == 0 {
        6
    } else {
        0
    }
}