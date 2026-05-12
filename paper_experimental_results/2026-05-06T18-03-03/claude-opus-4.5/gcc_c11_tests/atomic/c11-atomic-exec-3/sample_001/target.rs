use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::process::exit;

fn test_incdec() {
    // Helper macro for testing atomic increment/decrement
    macro_rules! test_incdec {
        // Pre-increment: ++a
        (pre_inc, $atomic_type:ty, $value_type:ty, $value:expr) => {{
            let a = <$atomic_type>::new($value as $value_type);
            let new_val = a.fetch_add(1, Ordering::SeqCst).wrapping_add(1);
            if new_val != ($value as $value_type).wrapping_add(1) {
                panic!("abort");
            }
            if a.load(Ordering::SeqCst) != ($value as $value_type).wrapping_add(1) {
                panic!("abort");
            }
        }};
        // Pre-decrement: --a
        (pre_dec, $atomic_type:ty, $value_type:ty, $value:expr) => {{
            let a = <$atomic_type>::new($value as $value_type);
            let new_val = a.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1);
            if new_val != ($value as $value_type).wrapping_sub(1) {
                panic!("abort");
            }
            if a.load(Ordering::SeqCst) != ($value as $value_type).wrapping_sub(1) {
                panic!("abort");
            }
        }};
        // Post-increment: a++
        (post_inc, $atomic_type:ty, $value_type:ty, $value:expr) => {{
            let a = <$atomic_type>::new($value as $value_type);
            let old_val = a.fetch_add(1, Ordering::SeqCst);
            if old_val != ($value as $value_type) {
                panic!("abort");
            }
            if a.load(Ordering::SeqCst) != ($value as $value_type).wrapping_add(1) {
                panic!("abort");
            }
        }};
        // Post-decrement: a--
        (post_dec, $atomic_type:ty, $value_type:ty, $value:expr) => {{
            let a = <$atomic_type>::new($value as $value_type);
            let old_val = a.fetch_sub(1, Ordering::SeqCst);
            if old_val != ($value as $value_type) {
                panic!("abort");
            }
            if a.load(Ordering::SeqCst) != ($value as $value_type).wrapping_sub(1) {
                panic!("abort");
            }
        }};
    }

    macro_rules! test_all_types {
        ($op:ident, $value:expr) => {{
            // bool: increment always sets to true (1), decrement from 0 wraps
            test_incdec!($op, AtomicBool, bool, if $value != 0 { true } else { false });
            test_incdec!($op, AtomicI8, i8, $value);
            test_incdec!($op, AtomicU8, u8, $value);
            test_incdec!($op, AtomicI16, i16, $value);
            test_incdec!($op, AtomicU16, u16, $value);
            test_incdec!($op, AtomicI32, i32, $value);
            test_incdec!($op, AtomicU32, u32, $value);
            test_incdec!($op, AtomicI64, i64, $value);
            test_incdec!($op, AtomicU64, u64, $value);
        }};
    }

    macro_rules! test_all_ops {
        ($value:expr) => {{
            test_all_types!(pre_inc, $value);
            test_all_types!(pre_dec, $value);
            test_all_types!(post_inc, $value);
            test_all_types!(post_dec, $value);
        }};
    }

    test_all_ops!(0i64);
    test_all_ops!(1i64);
    test_all_ops!(2i64);
    test_all_ops!(-1i64);
    test_all_ops!(1i64 << 60);

    // Test pointer arithmetic using AtomicUsize to simulate pointer offsets
    let ia: [i32; 2] = [0, 0];
    let base_ptr = ia.as_ptr() as usize;
    let ptr_to_ia1 = base_ptr + std::mem::size_of::<i32>();
    
    // Pre-increment pointer
    {
        let a = AtomicUsize::new(ptr_to_ia1);
        let new_val = a.fetch_add(std::mem::size_of::<i32>(), Ordering::SeqCst) + std::mem::size_of::<i32>();
        if new_val != ptr_to_ia1 + std::mem::size_of::<i32>() {
            panic!("abort");
        }
    }
    
    // Pre-decrement pointer
    {
        let a = AtomicUsize::new(ptr_to_ia1);
        let new_val = a.fetch_sub(std::mem::size_of::<i32>(), Ordering::SeqCst) - std::mem::size_of::<i32>();
        if new_val != ptr_to_ia1 - std::mem::size_of::<i32>() {
            panic!("abort");
        }
    }
    
    // Post-increment pointer
    {
        let a = AtomicUsize::new(ptr_to_ia1);
        let old_val = a.fetch_add(std::mem::size_of::<i32>(), Ordering::SeqCst);
        if old_val != ptr_to_ia1 {
            panic!("abort");
        }
    }
    
    // Post-decrement pointer
    {
        let a = AtomicUsize::new(ptr_to_ia1);
        let old_val = a.fetch_sub(std::mem::size_of::<i32>(), Ordering::SeqCst);
        if old_val != ptr_to_ia1 {
            panic!("abort");
        }
    }
}

fn main() {
    test_incdec();
    exit(0);
}