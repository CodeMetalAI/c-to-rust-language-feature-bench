struct Vector {
    n: i32,
    v: usize, // index into the pool
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
        H = H.wrapping_mul(0x5851f42d4c957f2du64);

        Some(p)
    }
}

fn read_float(offset: usize, index: usize) -> f32 {
    unsafe {
        let byte_offset = offset + index * std::mem::size_of::<f32>();
        let bytes: [u8; 4] = [
            POOL[byte_offset],
            POOL[byte_offset + 1],
            POOL[byte_offset + 2],
            POOL[byte_offset + 3],
        ];
        f32::from_ne_bytes(bytes)
    }
}

fn write_float(offset: usize, index: usize, value: f32) {
    unsafe {
        let byte_offset = offset + index * std::mem::size_of::<f32>();
        let bytes = value.to_ne_bytes();
        POOL[byte_offset] = bytes[0];
        POOL[byte_offset + 1] = bytes[1];
        POOL[byte_offset + 2] = bytes[2];
        POOL[byte_offset + 3] = bytes[3];
    }
}

fn new_vector(n: i32) -> (Vector, bool) {
    let size = (n as usize) * std::mem::size_of::<f32>();
    match alloc_bytes(size) {
        Some(v) => (Vector { n, v }, true),
        None => (Vector { n, v: 0 }, false),
    }
}

fn fill_vec(p: usize, n: i32, base: f32) {
    let mut i = 0;
    while i < n {
        write_float(p, i as usize, base + (i as f32));
        i += 1;
    }
}

fn sum_vec(p: usize, n: i32) -> f32 {
    let mut s = 0.0f32;
    let mut i = 0;
    while i < n {
        s += read_float(p, i as usize);
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
    let (a, a_valid) = new_vector(16);
    let (b, b_valid) = new_vector(16);

    if !a_valid {
        std::process::exit(1);
    }
    if !b_valid {
        std::process::exit(2);
    }

    if a.v == b.v {
        std::process::exit(3);
    }

    fill_vec(a.v, a.n, 1.0f32);
    fill_vec(b.v, b.n, 100.0f32);

    if !close_enough(sum_vec(a.v, a.n), 1.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32) {
        std::process::exit(4);
    }

    if !close_enough(sum_vec(b.v, b.n), 100.0f32 * 16.0f32 + (15 * 16) as f32 / 2.0f32) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}