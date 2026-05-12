use std::cell::RefCell;

type Usize = u64;

struct Vector {
    n: i32,
    v: Option<Box<[f32]>>,
}

static mut H: u64 = 0x9e3779b97f4a7c15u64;

thread_local! {
    static POOL: RefCell<([u8; 8192], usize)> = RefCell::new(([0u8; 8192], 0));
}

fn alloc_bytes(n: Usize) -> Option<*mut u8> {
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();
        let (ref mut memory, ref mut off) = *pool;

        if n == 0 {
            return Some((&mut memory[*off]) as *mut u8);
        }

        if *off + n as usize > memory.len() {
            return None;
        }

        let ptr = (&mut memory[*off]) as *mut u8;
        *off += n as usize;

        unsafe {
            H ^= (ptr as u64).wrapping_add(0x9e3779b97f4a7c15u64);
            H = H.wrapping_mul(0x5851f42d4c957f2du64);
        }

        Some(ptr)
    })
}

fn new_vector(n: i32) -> Vector {
    let size = (n as Usize).checked_mul(std::mem::size_of::<f32>() as Usize).unwrap_or(0);
    let ptr = alloc_bytes(size);
    
    let v = if let Some(raw_ptr) = ptr {
        if !raw_ptr.is_null() {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(raw_ptr as *mut f32, n as usize);
                Some(Box::from_raw(slice))
            }
        } else {
            None
        }
    } else {
        None
    };

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

    // Compare the underlying slice pointers
    let a_ptr = a.v.as_ref().unwrap().as_ptr();
    let b_ptr = b.v.as_ref().unwrap().as_ptr();
    if a_ptr == b_ptr {
        std::process::exit(3);
    }

    fill_vec(&mut a.v.as_ref().unwrap(), 1.0f32);
    fill_vec(&mut b.v.as_ref().unwrap(), 100.0f32);

    let sum_a = sum_vec(a.v.as_ref().unwrap());
    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_a, expected_a) {
        std::process::exit(4);
    }

    let sum_b = sum_vec(b.v.as_ref().unwrap());
    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_b, expected_b) {
        std::process::exit(5);
    }

    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}