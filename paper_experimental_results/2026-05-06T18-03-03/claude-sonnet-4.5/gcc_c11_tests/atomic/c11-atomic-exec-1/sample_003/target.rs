use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::cell::Cell;

fn main() {
    test_simple_assign();
    std::process::exit(0);
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith_float(1.5);
    test_simple_assign_arith_complex(2.5, 3.5);
    
    test_simple_assign_ptr();
    test_simple_assign_struct();
}

fn test_simple_assign_arith(value: i64) {
    test_simple_assign_bool(value != 0);
    test_simple_assign_i8(value as i8);
    test_simple_assign_i8(value as i8);
    test_simple_assign_u8(value as u8);
    test_simple_assign_i16(value as i16);
    test_simple_assign_u16(value as u16);
    test_simple_assign_i32(value as i32);
    test_simple_assign_u32(value as u32);
    test_simple_assign_isize(value as isize);
    test_simple_assign_usize(value as usize);
    test_simple_assign_i64(value);
    test_simple_assign_u64(value as u64);
}

fn test_simple_assign_arith_float(value: f64) {
    test_simple_assign_f32(value as f32);
    test_simple_assign_f64(value);
    test_simple_assign_f64(value);
}

fn test_simple_assign_arith_complex(real: f64, imag: f64) {
    test_simple_assign_complex_f32(real as f32, imag as f32);
    test_simple_assign_complex_f64(real, imag);
    test_simple_assign_complex_f64(real, imag);
}

fn test_simple_assign_bool(value: bool) {
    let a = AtomicBool::new(false);
    let b = AtomicBool::new(value);
    
    if a.load(Ordering::SeqCst) != false {
        std::process::abort();
    }
    if b.load(Ordering::SeqCst) != value {
        std::process::abort();
    }
    a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        std::process::abort();
    }
    if a.load(Ordering::SeqCst) != value {
        std::process::abort();
    }
}

macro_rules! test_simple_assign_int {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: $value_type) {
            let a = <$atomic_type>::new(0);
            let b = <$atomic_type>::new(value);
            
            if a.load(Ordering::SeqCst) != 0 {
                std::process::abort();
            }
            if b.load(Ordering::SeqCst) != value {
                std::process::abort();
            }
            a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
            if a.load(Ordering::SeqCst) != value {
                std::process::abort();
            }
            if a.load(Ordering::SeqCst) != value {
                std::process::abort();
            }
        }
    };
}

test_simple_assign_int!(test_simple_assign_i8, AtomicI8, i8);
test_simple_assign_int!(test_simple_assign_u8, AtomicU8, u8);
test_simple_assign_int!(test_simple_assign_i16, AtomicI16, i16);
test_simple_assign_int!(test_simple_assign_u16, AtomicU16, u16);
test_simple_assign_int!(test_simple_assign_i32, AtomicI32, i32);
test_simple_assign_int!(test_simple_assign_u32, AtomicU32, u32);
test_simple_assign_int!(test_simple_assign_isize, AtomicIsize, isize);
test_simple_assign_int!(test_simple_assign_usize, AtomicUsize, usize);
test_simple_assign_int!(test_simple_assign_i64, AtomicI64, i64);
test_simple_assign_int!(test_simple_assign_u64, AtomicU64, u64);

fn test_simple_assign_f32(value: f32) {
    let a = Cell::new(0.0f32);
    let b = Cell::new(value);
    
    if a.get() != 0.0 {
        std::process::abort();
    }
    if b.get() != value {
        std::process::abort();
    }
    a.set(b.get());
    if a.get() != value {
        std::process::abort();
    }
    if a.get() != value {
        std::process::abort();
    }
}

fn test_simple_assign_f64(value: f64) {
    let a = Cell::new(0.0f64);
    let b = Cell::new(value);
    
    if a.get() != 0.0 {
        std::process::abort();
    }
    if b.get() != value {
        std::process::abort();
    }
    a.set(b.get());
    if a.get() != value {
        std::process::abort();
    }
    if a.get() != value {
        std::process::abort();
    }
}

fn test_simple_assign_complex_f32(real: f32, imag: f32) {
    let a = Cell::new((0.0f32, 0.0f32));
    let b = Cell::new((real, imag));
    
    if a.get() != (0.0, 0.0) {
        std::process::abort();
    }
    if b.get() != (real, imag) {
        std::process::abort();
    }
    a.set(b.get());
    if a.get() != (real, imag) {
        std::process::abort();
    }
    if a.get() != (real, imag) {
        std::process::abort();
    }
}

fn test_simple_assign_complex_f64(real: f64, imag: f64) {
    let a = Cell::new((0.0f64, 0.0f64));
    let b = Cell::new((real, imag));
    
    if a.get() != (0.0, 0.0) {
        std::process::abort();
    }
    if b.get() != (real, imag) {
        std::process::abort();
    }
    a.set(b.get());
    if a.get() != (real, imag) {
        std::process::abort();
    }
    if a.get() != (real, imag) {
        std::process::abort();
    }
}

fn test_simple_assign_ptr() {
    static mut I: i32 = 0;
    
    let a = Cell::new(std::ptr::null::<i32>());
    let b = Cell::new(std::ptr::null::<i32>());
    
    if !a.get().is_null() {
        std::process::abort();
    }
    if !b.get().is_null() {
        std::process::abort();
    }
    a.set(b.get());
    if !a.get().is_null() {
        std::process::abort();
    }
    
    let b2 = Cell::new(unsafe { &I as *const i32 });
    let a2 = Cell::new(std::ptr::null::<i32>());
    
    if !a2.get().is_null() {
        std::process::abort();
    }
    if b2.get() != unsafe { &I as *const i32 } {
        std::process::abort();
    }
    a2.set(b2.get());
    if a2.get() != unsafe { &I as *const i32 } {
        std::process::abort();
    }
}

fn test_simple_assign_struct() {
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let s1 = Cell::new(S { a: [0; 1024] });
    let s2 = Cell::new(S { a: [0; 1024] });
    
    s1.set(init);
    let copy = s1.get();
    if copy.a != init.a {
        std::process::abort();
    }
    
    s2.set(s1.get());
    let copy = s2.get();
    if copy.a != init.a {
        std::process::abort();
    }
    
    let copy = s1.get();
    if copy.a != init.a {
        std::process::abort();
    }
    
    let copy = s2.get();
    if copy.a != init.a {
        std::process::abort();
    }
}