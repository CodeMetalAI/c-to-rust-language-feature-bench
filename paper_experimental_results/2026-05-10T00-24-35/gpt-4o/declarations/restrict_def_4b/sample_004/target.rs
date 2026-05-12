// restrict_def_4b.rs

type Usize = u64;

struct Vector {
    n: i32,
    v: &'static mut [f32],
}

static mut H: Usize = 0x9e3779b97f4a7c15;

fn alloc_bytes(n: Usize) -> Option<&'static mut [u8]> {
    static mut POOL: [u8; 8192] = [0; 8192];
    static mut OFF: Usize = 0;

    unsafe {
        if n == 0 {
            return Some(&mut POOL[OFF as usize..]);
        }

        if OFF + n > POOL.len() as Usize {
            return None;
        }

        let p = &mut POOL[OFF as usize..OFF as usize + n as usize];
        OFF += n;

        H ^= p.as_ptr() as Usize + 0x9e3779b97f4a7c15;
        H = H.wrapping_mul(0x5851f42d4c957f2d);

        Some(p)
    }
}

fn new_vector(n: i32) -> Option<Vector> {
    let n_bytes = n as Usize * std::mem::size_of::<f32>() as Usize;
    alloc_bytes(n_bytes).map(|slice| {
        let ptr = slice.as_mut_ptr() as *mut f32;
        let len = n_bytes / std::mem::size_of::<f32>() as Usize;
        Vector {
            n,
            v: unsafe { std::slice::from_raw_parts_mut(ptr, len as usize) },
        }
    })
}

fn fill_vec(p: &mut [f32], n: i32, base: f32) {
    for i in 0..n as usize {
        p[i] = base + i as f32;
    }
}

fn sum_vec(p: &[f32], n: i32) -> f32 {
    let mut s = 0.0f32;
    for i in 0..n as usize {
        s += p[i];
    }
    s
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = (x - y).abs();
    d < 0.0001f32
}

fn main() {
    let a = match new_vector(16) {
        Some(vec) => vec,
        None => {
            std::process::exit(1);
        }
    };
    let b = match new_vector(16) {
        Some(vec) => vec,
        None => {
            std::process::exit(2);
        }
    };

    if std::ptr::eq(a.v.as_ptr(), b.v.as_ptr()) {
        std::process::exit(3);
    }

    fill_vec(&mut a.v, a.n, 1.0f32);
    fill_vec(&mut b.v, b.n, 100.0f32);

    if !close_enough(
        sum_vec(&a.v, a.n),
        1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32,
    ) {
        std::process::exit(4);
    }

    if !close_enough(
        sum_vec(&b.v, b.n),
        100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32,
    ) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1) == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}