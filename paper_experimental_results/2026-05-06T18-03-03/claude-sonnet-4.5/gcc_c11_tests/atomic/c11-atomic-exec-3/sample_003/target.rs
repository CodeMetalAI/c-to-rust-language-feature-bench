use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

fn main() {
    test_incdec();
    process::exit(0);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith_float(1.5);
    
    // Pointer arithmetic test
    let ia = [0i32, 0i32];
    let ptr = &ia[1] as *const i32 as usize;
    test_incdec_ptr(ptr, true, true, std::mem::size_of::<i32>() as isize);
    test_incdec_ptr(ptr, true, false, -(std::mem::size_of::<i32>() as isize));
    test_incdec_ptr(ptr, false, true, std::mem::size_of::<i32>() as isize);
    test_incdec_ptr(ptr, false, false, -(std::mem::size_of::<i32>() as isize));
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, true, 1);
    test_incdec_arith(value, true, false, -1);
    test_incdec_arith(value, false, true, 1);
    test_incdec_arith(value, false, false, -1);
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_arith_float(value, true, true, 1.0);
    test_incdec_arith_float(value, true, false, -1.0);
    test_incdec_arith_float(value, false, true, 1.0);
    test_incdec_arith_float(value, false, false, -1.0);
}

fn test_incdec_arith(value: i64, pre_p: bool, increment: bool, change: i64) {
    test_incdec_bool(value != 0, pre_p, increment, change != 0);
    test_incdec_i8(value as i8, pre_p, increment, change as i8);
    test_incdec_u8(value as u8, pre_p, increment, change as u8);
    test_incdec_i16(value as i16, pre_p, increment, change as i16);
    test_incdec_u16(value as u16, pre_p, increment, change as u16);
    test_incdec_i32(value as i32, pre_p, increment, change as i32);
    test_incdec_u32(value as u32, pre_p, increment, change as u32);
    test_incdec_i64(value, pre_p, increment, change);
    test_incdec_u64(value as u64, pre_p, increment, change as u64);
}

fn test_incdec_arith_float(value: f64, pre_p: bool, increment: bool, change: f64) {
    test_incdec_f32(value as f32, pre_p, increment, change as f32);
    test_incdec_f64(value, pre_p, increment, change);
}

macro_rules! test_incdec_int {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: $value_type, pre_p: bool, increment: bool, change: $value_type) {
            let a = <$atomic_type>::new(value);
            let result = if increment {
                if pre_p {
                    a.fetch_add(change, Ordering::SeqCst).wrapping_add(change)
                } else {
                    a.fetch_add(change, Ordering::SeqCst)
                }
            } else {
                if pre_p {
                    a.fetch_sub(change.wrapping_neg(), Ordering::SeqCst).wrapping_sub(change.wrapping_neg())
                } else {
                    a.fetch_sub(change.wrapping_neg(), Ordering::SeqCst)
                }
            };
            let expected = if pre_p { value.wrapping_add(change) } else { value };
            if result != expected {
                process::abort();
            }
            if a.load(Ordering::SeqCst) != value.wrapping_add(change) {
                process::abort();
            }
        }
    };
}

test_incdec_int!(test_incdec_i8, AtomicI8, i8);
test_incdec_int!(test_incdec_u8, AtomicU8, u8);
test_incdec_int!(test_incdec_i16, AtomicI16, i16);
test_incdec_int!(test_incdec_u16, AtomicU16, u16);
test_incdec_int!(test_incdec_i32, AtomicI32, i32);
test_incdec_int!(test_incdec_u32, AtomicU32, u32);
test_incdec_int!(test_incdec_i64, AtomicI64, i64);
test_incdec_int!(test_incdec_u64, AtomicU64, u64);
test_incdec_int!(test_incdec_ptr, AtomicUsize, usize);

fn test_incdec_bool(value: bool, pre_p: bool, increment: bool, _change: bool) {
    let a = AtomicBool::new(value);
    let result_val = if increment {
        a.fetch_or(true, Ordering::SeqCst)
    } else {
        a.fetch_and(false, Ordering::SeqCst)
    };
    let expected = if pre_p { value || increment } else { value };
    if result_val != expected {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != (value || increment) {
        process::abort();
    }
}

fn test_incdec_f32(value: f32, pre_p: bool, increment: bool, change: f32) {
    let a = AtomicU32::new(value.to_bits());
    let old_bits = if increment {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |bits| {
            Some((f32::from_bits(bits) + change).to_bits())
        }).unwrap()
    } else {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |bits| {
            Some((f32::from_bits(bits) - change).to_bits())
        }).unwrap()
    };
    let result = if pre_p { f32::from_bits(old_bits) + if increment { change } else { -change } } else { f32::from_bits(old_bits) };
    let expected = if pre_p { value + if increment { change } else { -change } } else { value };
    if result != expected {
        process::abort();
    }
    if f32::from_bits(a.load(Ordering::SeqCst)) != value + if increment { change } else { -change } {
        process::abort();
    }
}

fn test_incdec_f64(value: f64, pre_p: bool, increment: bool, change: f64) {
    let a = AtomicU64::new(value.to_bits());
    let old_bits = if increment {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |bits| {
            Some((f64::from_bits(bits) + change).to_bits())
        }).unwrap()
    } else {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |bits| {
            Some((f64::from_bits(bits) - change).to_bits())
        }).unwrap()
    };
    let result = if pre_p { f64::from_bits(old_bits) + if increment { change } else { -change } } else { f64::from_bits(old_bits) };
    let expected = if pre_p { value + if increment { change } else { -change } } else { value };
    if result != expected {
        process::abort();
    }
    if f64::from_bits(a.load(Ordering::SeqCst)) != value + if increment { change } else { -change } {
        process::abort();
    }
}