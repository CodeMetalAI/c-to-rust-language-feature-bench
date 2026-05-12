use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
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
    test_simple_assign_i64(value as i64);
    test_simple_assign_u64(value as u64);
    test_simple_assign_i64(value as i64);
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
}

fn test_simple_assign_i8(value: i8) {
    let a = AtomicI8::new(0);
    let b = AtomicI8::new(value);
    
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
}

fn test_simple_assign_u8(value: u8) {
    let a = AtomicU8::new(0);
    let b = AtomicU8::new(value);
    
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
}

fn test_simple_assign_i16(value: i16) {
    let a = AtomicI16::new(0);
    let b = AtomicI16::new(value);
    
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
}

fn test_simple_assign_u16(value: u16) {
    let a = AtomicU16::new(0);
    let b = AtomicU16::new(value);
    
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
}

fn test_simple_assign_i32(value: i32) {
    let a = AtomicI32::new(0);
    let b = AtomicI32::new(value);
    
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
}

fn test_simple_assign_u32(value: u32) {
    let a = AtomicU32::new(0);
    let b = AtomicU32::new(value);
    
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
}

fn test_simple_assign_i64(value: i64) {
    let a = AtomicI64::new(0);
    let b = AtomicI64::new(value);
    
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
}

fn test_simple_assign_u64(value: u64) {
    let a = AtomicU64::new(0);
    let b = AtomicU64::new(value);
    
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
}

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
}

fn test_simple_assign_ptr() {
    static mut I: i32 = 0;
    
    let a = AtomicUsize::new(0);
    let b = AtomicUsize::new(0);
    
    if a.load(Ordering::SeqCst) != 0 {
        std::process::abort();
    }
    if b.load(Ordering::SeqCst) != 0 {
        std::process::abort();
    }
    a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != 0 {
        std::process::abort();
    }
    
    let ptr_val = unsafe { &I as *const i32 as usize };
    let a2 = AtomicUsize::new(0);
    let b2 = AtomicUsize::new(ptr_val);
    
    if a2.load(Ordering::SeqCst) != 0 {
        std::process::abort();
    }
    if b2.load(Ordering::SeqCst) != ptr_val {
        std::process::abort();
    }
    a2.store(b2.load(Ordering::SeqCst), Ordering::SeqCst);
    if a2.load(Ordering::SeqCst) != ptr_val {
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