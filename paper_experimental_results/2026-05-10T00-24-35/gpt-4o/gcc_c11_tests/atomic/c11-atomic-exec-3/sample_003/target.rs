use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::process;

fn test_incdec() {
    fn test_incdec_type<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T>>(
        initial_value: T,
        pre_op: bool,
        change: T,
        ordering: Ordering,
    ) where
        AtomicBool: From<T>,
        AtomicI32: From<T>,
        AtomicUsize: From<T>,
    {
        // Test with AtomicBool
        let a = AtomicBool::from(initial_value);
        let expected = if pre_op {
            initial_value + change
        } else {
            initial_value
        };

        if a.load(ordering) != expected {
            process::abort();
        }

        if a.load(ordering) != (initial_value + change) {
            process::abort();
        }

        // Test with AtomicI32
        let a = AtomicI32::from(initial_value);
        let expected = if pre_op {
            initial_value + change
        } else {
            initial_value
        };

        if a.load(ordering) != expected {
            process::abort();
        }

        if a.load(ordering) != (initial_value + change) {
            process::abort();
        }

        // Test with AtomicUsize
        let a = AtomicUsize::from(initial_value);
        let expected = if pre_op {
            initial_value + change
        } else {
            initial_value
        };

        if a.load(ordering) != expected {
            process::abort();
        }

        if a.load(ordering) != (initial_value + change) {
            process::abort();
        }
    }

    fn test_all_incdec_arith<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default>(
        value: T,
    ) where
        AtomicBool: From<T>,
        AtomicI32: From<T>,
        AtomicUsize: From<T>,
    {
        test_incdec_type(value, true, T::default() + T::default(), Ordering::SeqCst);
        test_incdec_type(value, true, T::default() - T::default(), Ordering::SeqCst);
        test_incdec_type(value, false, T::default() + T::default(), Ordering::SeqCst);
        test_incdec_type(value, false, T::default() - T::default(), Ordering::SeqCst);
    }

    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5);

    let ia = [0; 2];
    let ptr = &ia[1] as *const i32 as usize;

    test_incdec_type(ptr, true, 1, Ordering::SeqCst);
    test_incdec_type(ptr, true, -1, Ordering::SeqCst);
    test_incdec_type(ptr, false, 1, Ordering::SeqCst);
    test_incdec_type(ptr, false, -1, Ordering::SeqCst);
}

fn main() {
    test_incdec();
    process::exit(0);
}