use std::process::exit;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicIsize, AtomicUsize, AtomicI64, AtomicU64, AtomicPtr};
use std::sync::atomic::Ordering::SeqCst;
use std::ptr;

fn abort() {
    exit(1);
}

macro_rules! test_incdec {
    ($atomic:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        let a = $atomic;
        let initial = a.load(SeqCst);
        
        // Apply the operation based on prefix/postfix
        let result = match ($preop, $postop) {
            ("++", "") => {
                let old = a.fetch_add($change, SeqCst);
                if $pre_p { old.wrapping_add($change) } else { old }
            }
            ("--", "") => {
                let old = a.fetch_sub(-$change, SeqCst);
                if $pre_p { old.wrapping_sub(-$change) } else { old }
            }
            ("", "++") => {
                let old = a.fetch_add($change, SeqCst);
                if $pre_p { old.wrapping_add($change) } else { old }
            }
            ("", "--") => {
                let old = a.fetch_sub(-$change, SeqCst);
                if $pre_p { old.wrapping_sub(-$change) } else { old }
            }
            _ => unreachable!(),
        };
        
        let expected_result = if $pre_p {
            initial.wrapping_add($change)
        } else {
            initial
        };
        
        if result != expected_result {
            abort();
        }
        
        let final_val = a.load(SeqCst);
        let expected_final = initial.wrapping_add($change);
        if final_val != expected_final {
            abort();
        }
    }};
}

macro_rules! test_incdec_float {
    ($atomic:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        let a = $atomic;
        let initial = a.load(SeqCst);
        
        // For floats, we need to handle the operations differently
        let (result, final_val) = match ($preop, $postop) {
            ("++", "") => {
                let old = a.load(SeqCst);
                a.store(old + $change, SeqCst);
                if $pre_p { (old + $change, old + $change) } else { (old, old + $change) }
            }
            ("--", "") => {
                let old = a.load(SeqCst);
                a.store(old - (-$change), SeqCst);
                if $pre_p { (old - (-$change), old - (-$change)) } else { (old, old - (-$change)) }
            }
            ("", "++") => {
                let old = a.load(SeqCst);
                a.store(old + $change, SeqCst);
                if $pre_p { (old + $change, old + $change) } else { (old, old + $change) }
            }
            ("", "--") => {
                let old = a.load(SeqCst);
                a.store(old - (-$change), SeqCst);
                if $pre_p { (old - (-$change), old - (-$change)) } else { (old, old - (-$change)) }
            }
            _ => unreachable!(),
        };
        
        let expected_result = if $pre_p {
            initial + $change
        } else {
            initial
        };
        
        // Compare floats with tolerance
        if (result - expected_result).abs() > 1e-10 {
            abort();
        }
        
        let expected_final = initial + $change;
        if (final_val - expected_final).abs() > 1e-10 {
            abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        test_incdec!(AtomicBool::new($value != 0), $preop, $postop, $pre_p, $change as u8);
        test_incdec!(AtomicI8::new($value as i8), $preop, $postop, $pre_p, $change as i8);
        test_incdec!(AtomicU8::new($value as u8), $preop, $postop, $pre_p, $change as u8);
        test_incdec!(AtomicI16::new($value as i16), $preop, $postop, $pre_p, $change as i16);
        test_incdec!(AtomicU16::new($value as u16), $preop, $postop, $pre_p, $change as u16);
        test_incdec!(AtomicI32::new($value as i32), $preop, $postop, $pre_p, $change as i32);
        test_incdec!(AtomicU32::new($value as u32), $preop, $postop, $pre_p, $change as u32);
        test_incdec!(AtomicIsize::new($value as isize), $preop, $postop, $pre_p, $change as isize);
        test_incdec!(AtomicUsize::new($value as usize), $preop, $postop, $pre_p, $change as usize);
        test_incdec!(AtomicI64::new($value as i64), $preop, $postop, $pre_p, $change as i64);
        test_incdec!(AtomicU64::new($value as u64), $preop, $postop, $pre_p, $change as u64);
        
        // Handle floats separately since they don't have atomic fetch_add
        let atomic_f32 = std::sync::atomic::AtomicU32::new(($value as f32).to_bits());
        let atomic_f64 = std::sync::atomic::AtomicU64::new(($value as f64).to_bits());
        
        // For floats, we need to handle them differently
        {
            let a = std::sync::atomic::AtomicU32::new(($value as f32).to_bits());
            let initial = f32::from_bits(a.load(SeqCst));
            let (result, final_val) = match ($preop, $postop) {
                ("++", "") => {
                    let old = f32::from_bits(a.load(SeqCst));
                    let new_val = old + $change as f32;
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("--", "") => {
                    let old = f32::from_bits(a.load(SeqCst));
                    let new_val = old - (-$change as f32);
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("", "++") => {
                    let old = f32::from_bits(a.load(SeqCst));
                    let new_val = old + $change as f32;
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("", "--") => {
                    let old = f32::from_bits(a.load(SeqCst));
                    let new_val = old - (-$change as f32);
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                _ => unreachable!(),
            };
            
            let expected_result = if $pre_p {
                initial + $change as f32
            } else {
                initial
            };
            
            if (result - expected_result).abs() > 1e-6 {
                abort();
            }
            
            let expected_final = initial + $change as f32;
            if (final_val - expected_final).abs() > 1e-6 {
                abort();
            }
        }
        
        {
            let a = std::sync::atomic::AtomicU64::new(($value as f64).to_bits());
            let initial = f64::from_bits(a.load(SeqCst));
            let (result, final_val) = match ($preop, $postop) {
                ("++", "") => {
                    let old = f64::from_bits(a.load(SeqCst));
                    let new_val = old + $change as f64;
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("--", "") => {
                    let old = f64::from_bits(a.load(SeqCst));
                    let new_val = old - (-$change as f64);
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("", "++") => {
                    let old = f64::from_bits(a.load(SeqCst));
                    let new_val = old + $change as f64;
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                ("", "--") => {
                    let old = f64::from_bits(a.load(SeqCst));
                    let new_val = old - (-$change as f64);
                    a.store(new_val.to_bits(), SeqCst);
                    if $pre_p { (new_val, new_val) } else { (old, new_val) }
                }
                _ => unreachable!(),
            };
            
            let expected_result = if $pre_p {
                initial + $change as f64
            } else {
                initial
            };
            
            if (result - expected_result).abs() > 1e-10 {
                abort();
            }
            
            let expected_final = initial + $change as f64;
            if (final_val - expected_final).abs() > 1e-10 {
                abort();
            }
        }
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value, "++", "", true, 1);
        test_incdec_arith!($value, "--", "", true, -1);
        test_incdec_arith!($value, "", "++", false, 1);
        test_incdec_arith!($value, "", "--", false, -1);
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
    
    // Test pointer arithmetic
    let mut ia = [0i32; 2];
    let ptr1 = unsafe { ia.as_mut_ptr().add(1) };
    
    {
        let atomic_ptr = AtomicPtr::new(ptr1);
        test_incdec!(atomic_ptr, "++", "", true, 1);
    }
    
    {
        let atomic_ptr = AtomicPtr::new(ptr1);
        test_incdec!(atomic_ptr, "--", "", true, -1);
    }
    
    {
        let atomic_ptr = AtomicPtr::new(ptr1);
        test_incdec!(atomic_ptr, "", "++", false, 1);
    }
    
    {
        let atomic_ptr = AtomicPtr::new(ptr1);
        test_incdec!(atomic_ptr, "", "--", false, -1);
    }
}

fn main() {
    test_incdec();
    exit(0);
}