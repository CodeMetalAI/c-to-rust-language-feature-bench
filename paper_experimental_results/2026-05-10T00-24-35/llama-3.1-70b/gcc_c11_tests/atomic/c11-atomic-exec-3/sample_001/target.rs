use std::sync::atomic::{AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize};
use std::sync::atomic::Ordering;

fn test_incdec<T: Copy>(value: T, change: T, preop: bool, postop: bool) {
    let a = std::sync::atomic::AtomicPtr::new(&value);
    let result = if preop {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
            if postop {
                (x as *const T).offset(change as isize)
            } else {
                Some((x as *const T).offset(1))
            }
        }).unwrap()
    } else {
        a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
            if postop {
                Some((x as *const T).offset(change as isize))
            } else {
                (x as *const T).offset(1)
            }
        }).unwrap()
    };
    if result as *const T!= if preop {
        if postop {
            (value as *const T).offset(change as isize)
        } else {
            (value as *const T).offset(1)
        }
    } else {
        if postop {
            (value as *const T).offset(change as isize)
        } else {
            value as *const T
        }
    } {
        std::process::abort();
    }
    let new_value = a.load(Ordering::SeqCst);
    if new_value!= if preop {
        if postop {
            (value as *const T).offset(change as isize)
        } else {
            (value as *const T).offset(1)
        }
    } else {
        if postop {
            (value as *const T).offset(change as isize)
        } else {
            value as *const T
        }
    } {
        std::process::abort();
    }
}

fn test_all_incdec_arith<T: Copy>(value: T) {
    test_incdec(value, 1, true, false);
    test_incdec(value, -1, true, false);
    test_incdec(value, 1, false, true);
    test_incdec(value, -1, false, true);
}

fn test_incdec_arith<T: Copy>(value: T) {
    test_all_incdec_arith::<i8>(value as i8);
    test_all_incdec_arith::<u8>(value as u8);
    test_all_incdec_arith::<i16>(value as i16);
    test_all_incdec_arith::<u16>(value as u16);
    test_all_incdec_arith::<i32>(value as i32);
    test_all_incdec_arith::<u32>(value as u32);
    test_all_incdec_arith::<i64>(value as i64);
    test_all_incdec_arith::<u64>(value as u64);
    test_all_incdec_arith::<isize>(value as isize);
    test_all_incdec_arith::<usize>(value as usize);
}

fn test_incdec() {
    test_incdec_arith(0);
    test_incdec_arith(1);
    test_incdec_arith(2);
    test_incdec_arith(-1);
    test_incdec_arith(1u64 << 60);
    test_incdec_arith(1.5 as f64);
    let ia = [0, 0];
    test_incdec(&ia[1], 1, true, false);
    test_incdec(&ia[1], -1, true, false);
    test_incdec(&ia[1], 1, false, true);
    test_incdec(&ia[1], -1, false, true);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}