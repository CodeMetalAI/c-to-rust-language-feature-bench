use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process;

fn test_atomic_bool(value: bool) {
    let a = AtomicBool::new(false);
    let b = AtomicBool::new(value);
    
    if a.load(Ordering::SeqCst) != false {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_i8(value: i8) {
    let a = AtomicI8::new(0);
    let b = AtomicI8::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_u8(value: u8) {
    let a = AtomicU8::new(0);
    let b = AtomicU8::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_i16(value: i16) {
    let a = AtomicI16::new(0);
    let b = AtomicI16::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_u16(value: u16) {
    let a = AtomicU16::new(0);
    let b = AtomicU16::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_i32(value: i32) {
    let a = AtomicI32::new(0);
    let b = AtomicI32::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_u32(value: u32) {
    let a = AtomicU32::new(0);
    let b = AtomicU32::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_i64(value: i64) {
    let a = AtomicI64::new(0);
    let b = AtomicI64::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_atomic_u64(value: u64) {
    let a = AtomicU64::new(0);
    let b = AtomicU64::new(value);
    
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        process::abort();
    }
    let loaded = b.load(Ordering::SeqCst);
    a.store(loaded, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        process::abort();
    }
}

fn test_simple_assign() {
    for &val in &[0i64, 1, 2, -1, 1i64 << 63] {
        test_atomic_bool(val != 0);
        test_atomic_i8(val as i8);
        test_atomic_u8(val as u8);
        test_atomic_i16(val as i16);
        test_atomic_u16(val as u16);
        test_atomic_i32(val as i32);
        test_atomic_u32(val as u32);
        test_atomic_i64(val);
        test_atomic_u64(val as u64);
    }
    
    let init: [i16; 1024] = core::array::from_fn(|i| i as i16);
    let copy = init;
    if copy != init {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}