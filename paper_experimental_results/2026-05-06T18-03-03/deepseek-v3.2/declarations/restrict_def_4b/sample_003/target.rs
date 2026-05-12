use std::cell::RefCell;

type Usize = u64;

struct Vector {
    n: i32,
    v: Option<Box<[f32]>>,
}

static mut H: u64 = 0x9e3779b97f4a7c15u64;

thread_local! {
    static POOL: RefCell<([u8; 8192], Usize)> = RefCell::new(([0u8; 8192], 0));
}

fn alloc_bytes(n: Usize) -> Option<*mut u8> {
    POOL.with(|pool_cell| {
        let (ref pool, ref mut off) = *pool_cell.borrow_mut();
        
        if n == 0 {
            return Some(unsafe { pool.as_ptr().add(*off) } as *mut u8);
        }
        
        if *off + n > pool.len() as Usize {
            return None;
        }
        
        let p = unsafe { pool.as_ptr().add(*off) } as *mut u8;
        *off += n;
        
        unsafe {
            H ^= (p as Usize).wrapping_add(0x9e3779b97f4a7c15u64);
            H = H.wrapping_mul(0x5851f42d4c957f2du64);
        }
        
        Some(p)
    })
}

fn new_vector(n: i32) -> Vector {
    let size = (n as Usize).checked_mul(std::mem::size_of::<f32>() as Usize).unwrap_or(0);
    let ptr = alloc_bytes(size);
    
    let v = if let Some(ptr) = ptr {
        if ptr.is_null() {
            None
        } else {
            let slice_ptr = ptr as *mut f32;
            let slice = unsafe { std::slice::from_raw_parts_mut(slice_ptr, n as usize) };
            Some(unsafe { Box::from_raw(slice) })
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
    let d = (x - y).abs();
    d < 0.0001f32
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
    
    // Compare the underlying pointers
    let a_ptr = a.v.as_ref().unwrap().as_ptr() as *const u8;
    let b_ptr = b.v.as_ref().unwrap().as_ptr() as *const u8;
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