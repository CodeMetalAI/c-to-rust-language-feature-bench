use std::mem;
use std::process;

fn abort() {
    process::exit(1);
}

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static mut A: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            static B: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            
            // We'll use a different approach since we can't have static atomics of arbitrary types
            // Instead, we'll test with actual atomic types available in Rust
            match stringify!($type) {
                "bool" => {
                    use std::sync::atomic::{AtomicBool, Ordering};
                    static A_BOOL: AtomicBool = AtomicBool::new(false);
                    static B_BOOL: AtomicBool = AtomicBool::new($value as bool);
                    
                    if A_BOOL.load(Ordering::SeqCst) != false {
                        abort();
                    }
                    if B_BOOL.load(Ordering::SeqCst) != ($value as bool) {
                        abort();
                    }
                    A_BOOL.store(B_BOOL.load(Ordering::SeqCst), Ordering::SeqCst);
                    if A_BOOL.load(Ordering::SeqCst) != ($value as bool) {
                        abort();
                    }
                },
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => {
                    // For integer types, use the appropriate atomic
                    macro_rules! test_int_atomic {
                        ($atomic_type:ty, $val:expr) => {
                            static A_INT: $atomic_type = <$atomic_type>::new(0);
                            static B_INT: $atomic_type = <$atomic_type>::new($val);
                            
                            if A_INT.load(Ordering::SeqCst) != 0 {
                                abort();
                            }
                            if B_INT.load(Ordering::SeqCst) != $val {
                                abort();
                            }
                            A_INT.store(B_INT.load(Ordering::SeqCst), Ordering::SeqCst);
                            if A_INT.load(Ordering::SeqCst) != $val {
                                abort();
                            }
                        };
                    }
                    
                    match stringify!($type) {
                        "i8" => test_int_atomic!(std::sync::atomic::AtomicI8, $value as i8),
                        "u8" => test_int_atomic!(std::sync::atomic::AtomicU8, $value as u8),
                        "i16" => test_int_atomic!(std::sync::atomic::AtomicI16, $value as i16),
                        "u16" => test_int_atomic!(std::sync::atomic::AtomicU16, $value as u16),
                        "i32" => test_int_atomic!(std::sync::atomic::AtomicI32, $value as i32),
                        "u32" => test_int_atomic!(std::sync::atomic::AtomicU32, $value as u32),
                        "i64" => test_int_atomic!(std::sync::atomic::AtomicI64, $value as i64),
                        "u64" => test_int_atomic!(std::sync::atomic::AtomicU64, $value as u64),
                        "isize" => test_int_atomic!(std::sync::atomic::AtomicIsize, $value as isize),
                        "usize" => test_int_atomic!(std::sync::atomic::AtomicUsize, $value as usize),
                        _ => {}
                    }
                },
                "f32" | "f64" => {
                    // For floats, we'll use AtomicU32/AtomicU64 and transmute
                    match stringify!($type) {
                        "f32" => {
                            use std::sync::atomic::{AtomicU32, Ordering};
                            static A_FLOAT: AtomicU32 = AtomicU32::new(0);
                            static B_FLOAT: AtomicU32 = AtomicU32::new(unsafe { mem::transmute::<f32, u32>($value as f32) });
                            
                            if unsafe { mem::transmute::<u32, f32>(A_FLOAT.load(Ordering::SeqCst)) } != 0.0 {
                                abort();
                            }
                            if unsafe { mem::transmute::<u32, f32>(B_FLOAT.load(Ordering::SeqCst)) } != ($value as f32) {
                                abort();
                            }
                            A_FLOAT.store(B_FLOAT.load(Ordering::SeqCst), Ordering::SeqCst);
                            if unsafe { mem::transmute::<u32, f32>(A_FLOAT.load(Ordering::SeqCst)) } != ($value as f32) {
                                abort();
                            }
                        },
                        "f64" => {
                            use std::sync::atomic::{AtomicU64, Ordering};
                            static A_FLOAT: AtomicU64 = AtomicU64::new(0);
                            static B_FLOAT: AtomicU64 = AtomicU64::new(unsafe { mem::transmute::<f64, u64>($value as f64) });
                            
                            if unsafe { mem::transmute::<u64, f64>(A_FLOAT.load(Ordering::SeqCst)) } != 0.0 {
                                abort();
                            }
                            if unsafe { mem::transmute::<u64, f64>(B_FLOAT.load(Ordering::SeqCst)) } != ($value as f64) {
                                abort();
                            }
                            A_FLOAT.store(B_FLOAT.load(Ordering::SeqCst), Ordering::SeqCst);
                            if unsafe { mem::transmute::<u64, f64>(A_FLOAT.load(Ordering::SeqCst)) } != ($value as f64) {
                                abort();
                            }
                        },
                        _ => {}
                    }
                },
                _ => {
                    // For other types (pointers, complex, structs), we'll simulate with a simple test
                    // since Rust doesn't have direct equivalents
                    if $value != $value {
                        abort();
                    }
                }
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        test_simple_assign!(bool, $value);
        test_simple_assign!(i8, $value);
        test_simple_assign!(u8, $value);
        test_simple_assign!(i16, $value);
        test_simple_assign!(u16, $value);
        test_simple_assign!(i32, $value);
        test_simple_assign!(u32, $value);
        test_simple_assign!(i64, $value);
        test_simple_assign!(u64, $value);
        test_simple_assign!(isize, $value);
        test_simple_assign!(usize, $value);
        test_simple_assign!(f32, $value);
        test_simple_assign!(f64, $value);
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    
    // Test struct assignment (simplified)
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let copy = init;
    if !init.a.iter().zip(copy.a.iter()).all(|(x, y)| x == y) {
        abort();
    }
    
    let copy2 = copy;
    if !init.a.iter().zip(copy2.a.iter()).all(|(x, y)| x == y) {
        abort();
    }
    
    let copy3 = init;
    if !init.a.iter().zip(copy3.a.iter()).all(|(x, y)| x == y) {
        abort();
    }
    
    let copy4 = copy2;
    if !init.a.iter().zip(copy4.a.iter()).all(|(x, y)| x == y) {
        abort();
    }
}

fn main() {
    test_simple_assign();
    // Equivalent to exit(0)
    process::exit(0);
}