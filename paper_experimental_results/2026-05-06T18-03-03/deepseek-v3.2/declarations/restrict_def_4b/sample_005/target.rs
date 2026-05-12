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

fn alloc_bytes(n: usize) -> Option<*mut u8> {
    POOL.with(|pool_cell| {
        let mut pool = pool_cell.borrow_mut();
        let (ref mut memory, ref mut off) = *pool;
        
        if n == 0 {
            return Some(unsafe { memory.as_mut_ptr().add(*off) });
        }
        
        if *off + n > memory.len() {
            return None;
        }
        
        let ptr = unsafe { memory.as_mut_ptr().add(*off) };
        *off += n;
        
        unsafe {
            H ^= (ptr as u64).wrapping_add(0x9e3779b97f4a7c15u64);
            H = H.wrapping_mul(0x5851f42d4c957f2du64);
        }
        
        Some(ptr)
    })
}

fn new_vector(n: i32) -> Vector {
    let size = (n as usize).checked_mul(std::mem::size_of::<f32>()).unwrap_or(0);
    let ptr = alloc_bytes(size);
    
    let v = if ptr.is_none() {
        None
    } else {
        // SAFETY: We just allocated enough space for n f32s
        let slice = unsafe {
            std::slice::from_raw_parts_mut(ptr.unwrap() as *mut f32, n as usize)
        };
        // Initialize to zero to match C behavior (uninitialized memory could be anything)
        for elem in slice.iter_mut() {
            *elem = 0.0;
        }
        Some(unsafe {
            Box::from_raw(slice as *mut [f32])
        })
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
    
    // Compare the underlying pointers
    let a_ptr = a.v.as_ref().unwrap().as_ptr() as *const u8;
    let b_ptr = b.v.as_ref().unwrap().as_ptr() as *const u8;
    if a_ptr == b_ptr {
        std::process::exit(3);
    }
    
    fill_vec(&mut a.v.as_ref().unwrap(), 1.0f32);
    fill_vec(&mut b.v.as_ref().unwrap(), 100.0f32);
    
    let expected_a = 1.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(&a.v.as_ref().unwrap()), expected_a) {
        std::process::exit(4);
    }
    
    let expected_b = 100.0f32 * 16.0f32 + (15.0f32 * 16.0f32) / 2.0f32;
    if !close_enough(sum_vec(&b.v.as_ref().unwrap()), expected_b) {
        std::process::exit(5);
    }
    
    unsafe {
        if (H & 1u64) == 0u64 {
            std::process::exit(6);
        }
    }
    
    std::process::exit(0);
}