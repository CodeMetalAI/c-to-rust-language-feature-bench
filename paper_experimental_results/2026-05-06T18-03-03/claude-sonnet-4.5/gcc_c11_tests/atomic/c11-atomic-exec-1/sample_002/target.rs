use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::cell::Cell;

fn memcmp(a: &[u8], b: &[u8]) -> i32 {
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            return if a[i] < b[i] { -1 } else { 1 };
        }
    }
    0
}

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        static A: $atomic_type = <$atomic_type>::new(0 as $value_type);
        static B: $atomic_type = <$atomic_type>::new($value as $value_type);
        
        if A.load(Ordering::SeqCst) != 0 as $value_type {
            std::process::abort();
        }
        if B.load(Ordering::SeqCst) != ($value as $value_type) {
            std::process::abort();
        }
        let b_val = B.load(Ordering::SeqCst);
        A.store(b_val, Ordering::SeqCst);
        if b_val != ($value as $value_type) {
            std::process::abort();
        }
        if A.load(Ordering::SeqCst) != ($value as $value_type) {
            std::process::abort();
        }
    }};
}

macro_rules! test_simple_assign_cell {
    ($value_type:ty, $value:expr) => {{
        thread_local! {
            static A: Cell<$value_type> = Cell::new(0.0);
            static B: Cell<$value_type> = Cell::new($value as $value_type);
        }
        
        A.with(|a| {
            if a.get() != 0.0 {
                std::process::abort();
            }
        });
        B.with(|b| {
            if b.get() != ($value as $value_type) {
                std::process::abort();
            }
            let b_val = b.get();
            A.with(|a| {
                a.set(b_val);
                if b_val != ($value as $value_type) {
                    std::process::abort();
                }
                if a.get() != ($value as $value_type) {
                    std::process::abort();
                }
            });
        });
    }};
}

fn test_simple_assign() {
    for value in &[0i64, 1, 2, -1, 1i64 << 63] {
        let v = *value;
        test_simple_assign_atomic!(AtomicBool, bool, v != 0);
        test_simple_assign_atomic!(AtomicI8, i8, v);
        test_simple_assign_atomic!(AtomicI8, i8, v);
        test_simple_assign_atomic!(AtomicU8, u8, v);
        test_simple_assign_atomic!(AtomicI16, i16, v);
        test_simple_assign_atomic!(AtomicU16, u16, v);
        test_simple_assign_atomic!(AtomicI32, i32, v);
        test_simple_assign_atomic!(AtomicU32, u32, v);
        test_simple_assign_atomic!(AtomicIsize, isize, v);
        test_simple_assign_atomic!(AtomicUsize, usize, v);
        test_simple_assign_atomic!(AtomicI64, i64, v);
        test_simple_assign_atomic!(AtomicU64, u64, v);
        test_simple_assign_cell!(f32, v);
        test_simple_assign_cell!(f64, v);
    }
    
    for value in &[1.5f64] {
        let v = *value;
        test_simple_assign_cell!(f32, v);
        test_simple_assign_cell!(f64, v);
    }

    let init = [0i16; 1024];
    let mut init_struct = init;
    for j in 0..1024 {
        init_struct[j] = j as i16;
    }
    
    thread_local! {
        static S1: Cell<[i16; 1024]> = Cell::new([0; 1024]);
        static S2: Cell<[i16; 1024]> = Cell::new([0; 1024]);
    }
    
    S1.with(|s1| {
        s1.set(init_struct);
        let copy = s1.get();
        let init_bytes = unsafe { std::slice::from_raw_parts(init_struct.as_ptr() as *const u8, std::mem::size_of_val(&init_struct)) };
        let copy_bytes = unsafe { std::slice::from_raw_parts(copy.as_ptr() as *const u8, std::mem::size_of_val(&copy)) };
        if memcmp(init_bytes, copy_bytes) != 0 {
            std::process::abort();
        }
        
        S2.with(|s2| {
            s2.set(s1.get());
            let copy2 = s2.get();
            let copy2_bytes = unsafe { std::slice::from_raw_parts(copy2.as_ptr() as *const u8, std::mem::size_of_val(&copy2)) };
            if memcmp(init_bytes, copy2_bytes) != 0 {
                std::process::abort();
            }
            
            let copy3 = s1.get();
            let copy3_bytes = unsafe { std::slice::from_raw_parts(copy3.as_ptr() as *const u8, std::mem::size_of_val(&copy3)) };
            if memcmp(init_bytes, copy3_bytes) != 0 {
                std::process::abort();
            }
            
            let copy4 = s2.get();
            let copy4_bytes = unsafe { std::slice::from_raw_parts(copy4.as_ptr() as *const u8, std::mem::size_of_val(&copy4)) };
            if memcmp(init_bytes, copy4_bytes) != 0 {
                std::process::abort();
            }
        });
    });
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}