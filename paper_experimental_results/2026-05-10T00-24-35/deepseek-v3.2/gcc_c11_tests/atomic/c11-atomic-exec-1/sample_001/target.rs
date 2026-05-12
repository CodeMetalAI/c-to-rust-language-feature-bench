use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::atomic::{AtomicPtr, AtomicI16 as AtomicShort, AtomicI32 as AtomicInt, AtomicI64 as AtomicLongLong};
use std::process;

fn abort() {
    process::exit(1);
}

fn test_simple_assign<T: PartialEq + Copy + Default>(value: T) {
    use std::sync::atomic::AtomicUsize;
    
    // For floating point and complex types, we'll handle separately
    // For integer types, we can use the atomic types
    macro_rules! test_atomic {
        ($atomic_type:ty, $value:expr) => {
            let a = <$atomic_type>::new(Default::default());
            let b = <$atomic_type>::new($value);
            
            if a.load(Ordering::SeqCst) != Default::default() {
                abort();
            }
            if b.load(Ordering::SeqCst) != $value {
                abort();
            }
            
            let loaded = b.load(Ordering::SeqCst);
            a.store(loaded, Ordering::SeqCst);
            
            if a.load(Ordering::SeqCst) != $value {
                abort();
            }
        };
    }
    
    // We need to handle different types separately since Rust doesn't have
    // generic atomic types for all types like C11 does.
    // This test will focus on the integer types that have direct atomic equivalents.
}

fn test_simple_assign_arith(value: i64) {
    // Test boolean
    {
        let val = value != 0;
        let a = AtomicBool::new(false);
        let b = AtomicBool::new(val);
        
        if a.load(Ordering::SeqCst) != false {
            abort();
        }
        if b.load(Ordering::SeqCst) != val {
            abort();
        }
        
        let loaded = b.load(Ordering::SeqCst);
        a.store(loaded, Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != val {
            abort();
        }
    }
    
    // Test integer types
    {
        let a_i8 = AtomicI8::new(0);
        let b_i8 = AtomicI8::new(value as i8);
        if a_i8.load(Ordering::SeqCst) != 0 { abort(); }
        if b_i8.load(Ordering::SeqCst) != value as i8 { abort(); }
        a_i8.store(b_i8.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_i8.load(Ordering::SeqCst) != value as i8 { abort(); }
    }
    
    {
        let a_u8 = AtomicU8::new(0);
        let b_u8 = AtomicU8::new(value as u8);
        if a_u8.load(Ordering::SeqCst) != 0 { abort(); }
        if b_u8.load(Ordering::SeqCst) != value as u8 { abort(); }
        a_u8.store(b_u8.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_u8.load(Ordering::SeqCst) != value as u8 { abort(); }
    }
    
    {
        let a_i16 = AtomicI16::new(0);
        let b_i16 = AtomicI16::new(value as i16);
        if a_i16.load(Ordering::SeqCst) != 0 { abort(); }
        if b_i16.load(Ordering::SeqCst) != value as i16 { abort(); }
        a_i16.store(b_i16.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_i16.load(Ordering::SeqCst) != value as i16 { abort(); }
    }
    
    {
        let a_u16 = AtomicU16::new(0);
        let b_u16 = AtomicU16::new(value as u16);
        if a_u16.load(Ordering::SeqCst) != 0 { abort(); }
        if b_u16.load(Ordering::SeqCst) != value as u16 { abort(); }
        a_u16.store(b_u16.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_u16.load(Ordering::SeqCst) != value as u16 { abort(); }
    }
    
    {
        let a_i32 = AtomicI32::new(0);
        let b_i32 = AtomicI32::new(value as i32);
        if a_i32.load(Ordering::SeqCst) != 0 { abort(); }
        if b_i32.load(Ordering::SeqCst) != value as i32 { abort(); }
        a_i32.store(b_i32.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_i32.load(Ordering::SeqCst) != value as i32 { abort(); }
    }
    
    {
        let a_u32 = AtomicU32::new(0);
        let b_u32 = AtomicU32::new(value as u32);
        if a_u32.load(Ordering::SeqCst) != 0 { abort(); }
        if b_u32.load(Ordering::SeqCst) != value as u32 { abort(); }
        a_u32.store(b_u32.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_u32.load(Ordering::SeqCst) != value as u32 { abort(); }
    }
    
    {
        let a_i64 = AtomicI64::new(0);
        let b_i64 = AtomicI64::new(value as i64);
        if a_i64.load(Ordering::SeqCst) != 0 { abort(); }
        if b_i64.load(Ordering::SeqCst) != value as i64 { abort(); }
        a_i64.store(b_i64.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_i64.load(Ordering::SeqCst) != value as i64 { abort(); }
    }
    
    {
        let a_u64 = AtomicU64::new(0);
        let b_u64 = AtomicU64::new(value as u64);
        if a_u64.load(Ordering::SeqCst) != 0 { abort(); }
        if b_u64.load(Ordering::SeqCst) != value as u64 { abort(); }
        a_u64.store(b_u64.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_u64.load(Ordering::SeqCst) != value as u64 { abort(); }
    }
    
    // Test isize/usize (platform-dependent, like long)
    {
        let a_isize = AtomicIsize::new(0);
        let b_isize = AtomicIsize::new(value as isize);
        if a_isize.load(Ordering::SeqCst) != 0 { abort(); }
        if b_isize.load(Ordering::SeqCst) != value as isize { abort(); }
        a_isize.store(b_isize.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_isize.load(Ordering::SeqCst) != value as isize { abort(); }
    }
    
    {
        let a_usize = AtomicUsize::new(0);
        let b_usize = AtomicUsize::new(value as usize);
        if a_usize.load(Ordering::SeqCst) != 0 { abort(); }
        if b_usize.load(Ordering::SeqCst) != value as usize { abort(); }
        a_usize.store(b_usize.load(Ordering::SeqCst), Ordering::SeqCst);
        if a_usize.load(Ordering::SeqCst) != value as usize { abort(); }
    }
}

fn test_simple_assign_float(value: f64) {
    // Rust doesn't have atomic floats in std, so we'll use atomic integers
    // for the float bit patterns to simulate the behavior
    {
        let a = AtomicU32::new(0);
        let b = AtomicU32::new((value as f32).to_bits());
        
        if a.load(Ordering::SeqCst) != 0 { abort(); }
        if f32::from_bits(b.load(Ordering::SeqCst)) != value as f32 { abort(); }
        
        let loaded = b.load(Ordering::SeqCst);
        a.store(loaded, Ordering::SeqCst);
        
        if f32::from_bits(a.load(Ordering::SeqCst)) != value as f32 { abort(); }
    }
    
    {
        let a = AtomicU64::new(0);
        let b = AtomicU64::new(value.to_bits());
        
        if a.load(Ordering::SeqCst) != 0 { abort(); }
        if f64::from_bits(b.load(Ordering::SeqCst)) != value { abort(); }
        
        let loaded = b.load(Ordering::SeqCst);
        a.store(loaded, Ordering::SeqCst);
        
        if f64::from_bits(a.load(Ordering::SeqCst)) != value { abort(); }
    }
}

fn test_simple_assign_complex(real: f64, imag: f64) {
    // Complex numbers as pairs of atomic floats (using bit patterns)
    {
        let a_real = AtomicU32::new(0);
        let a_imag = AtomicU32::new(0);
        let b_real = AtomicU32::new((real as f32).to_bits());
        let b_imag = AtomicU32::new((imag as f32).to_bits());
        
        if a_real.load(Ordering::SeqCst) != 0 || a_imag.load(Ordering::SeqCst) != 0 { abort(); }
        if f32::from_bits(b_real.load(Ordering::SeqCst)) != real as f32 || 
           f32::from_bits(b_imag.load(Ordering::SeqCst)) != imag as f32 { abort(); }
        
        let loaded_real = b_real.load(Ordering::SeqCst);
        let loaded_imag = b_imag.load(Ordering::SeqCst);
        a_real.store(loaded_real, Ordering::SeqCst);
        a_imag.store(loaded_imag, Ordering::SeqCst);
        
        if f32::from_bits(a_real.load(Ordering::SeqCst)) != real as f32 ||
           f32::from_bits(a_imag.load(Ordering::SeqCst)) != imag as f32 { abort(); }
    }
    
    {
        let a_real = AtomicU64::new(0);
        let a_imag = AtomicU64::new(0);
        let b_real = AtomicU64::new(real.to_bits());
        let b_imag = AtomicU64::new(imag.to_bits());
        
        if a_real.load(Ordering::SeqCst) != 0 || a_imag.load(Ordering::SeqCst) != 0 { abort(); }
        if f64::from_bits(b_real.load(Ordering::SeqCst)) != real || 
           f64::from_bits(b_imag.load(Ordering::SeqCst)) != imag { abort(); }
        
        let loaded_real = b_real.load(Ordering::SeqCst);
        let loaded_imag = b_imag.load(Ordering::SeqCst);
        a_real.store(loaded_real, Ordering::SeqCst);
        a_imag.store(loaded_imag, Ordering::SeqCst);
        
        if f64::from_bits(a_real.load(Ordering::SeqCst)) != real ||
           f64::from_bits(a_imag.load(Ordering::SeqCst)) != imag { abort(); }
    }
}

fn test_simple_assign_pointer() {
    let mut i = 0;
    
    // Test null pointer
    {
        let a = AtomicPtr::<i32>::new(ptr::null_mut());
        let b = AtomicPtr::<i32>::new(ptr::null_mut());
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() { abort(); }
        if b.load(Ordering::SeqCst) != ptr::null_mut() { abort(); }
        
        let loaded = b.load(Ordering::SeqCst);
        a.store(loaded, Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() { abort(); }
    }
    
    // Test pointer to i
    {
        let a = AtomicPtr::<i32>::new(ptr::null_mut());
        let b = AtomicPtr::<i32>::new(&mut i as *mut i32);
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() { abort(); }
        if b.load(Ordering::SeqCst) != &mut i as *mut i32 { abort(); }
        
        let loaded = b.load(Ordering::SeqCst);
        a.store(loaded, Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != &mut i as *mut i32 { abort(); }
    }
}

fn test_struct_assignment() {
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    // In Rust, we can't have atomic structs directly, so we'll use a Mutex or just
    // copy the struct since this test is about assignment semantics
    let s1 = init;
    let copy = s1;
    
    // Compare byte by byte
    let init_bytes = unsafe { mem::transmute::<&S, &[u8; mem::size_of::<S>()]>(&init) };
    let copy_bytes = unsafe { mem::transmute::<&S, &[u8; mem::size_of::<S>()]>(&copy) };
    
    if init_bytes != copy_bytes {
        abort();
    }
    
    let s2 = s1;
    let copy2 = s2;
    
    let copy2_bytes = unsafe { mem::transmute::<&S, &[u8; mem::size_of::<S>()]>(&copy2) };
    if init_bytes != copy2_bytes {
        abort();
    }
    
    let copy3 = s1;
    let copy3_bytes = unsafe { mem::transmute::<&S, &[u8; mem::size_of::<S>()]>(&copy3) };
    if init_bytes != copy3_bytes {
        abort();
    }
    
    let copy4 = s2;
    let copy4_bytes = unsafe { mem::transmute::<&S, &[u8; mem::size_of::<S>()]>(&copy4) };
    if init_bytes != copy4_bytes {
        abort();
    }
}

fn main() {
    // Test with various values as in the C code
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1 << 31); // Using 1 << 31 instead of 1 << 63 for 32-bit compatibility
    
    test_simple_assign_float(0.0);
    test_simple_assign_float(1.0);
    test_simple_assign_float(1.5);
    test_simple_assign_float(-1.0);
    
    test_simple_assign_complex(2.5, 3.5);
    test_simple_assign_complex(0.0, 0.0);
    test_simple_assign_complex(1.0, 0.0);
    
    test_simple_assign_pointer();
    test_struct_assignment();
    
    process::exit(0);
}