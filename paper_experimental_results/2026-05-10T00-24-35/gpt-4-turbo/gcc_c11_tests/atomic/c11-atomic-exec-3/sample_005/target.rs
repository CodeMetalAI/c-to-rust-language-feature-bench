fn main() {
    test_incdec();
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5);
    let mut ia = [0; 2];
    test_incdec_primitive(&mut ia[1], 1, true);
    test_incdec_primitive(&mut ia[1], -1, true);
    test_incdec_primitive(&mut ia[1], 1, false);
    test_incdec_primitive(&mut ia[1], -1, false);
}

fn test_all_incdec_arith<T: atomic::AtomicArithmetic + Copy>(value: T) {
    test_incdec_arith(value, 1, true);
    test_incdec_arith(value, -1, true);
    test_incdec_arith(value, 1, false);
    test_incdec_arith(value, -1, false);
}

fn test_incdec_arith<T>(value: T, change: T, pre_op: bool) 
    where T: atomic::AtomicArithmetic + Copy {
    test_incdec_atomic(value, change, pre_op);
}

fn test_incdec_atomic<T: atomic::AtomicArithmetic + Copy>(value: T, change: T, pre_op: bool) {
    let mut a = std::sync::atomic::Atomic::new(value);
    let result = if pre_op {
        a.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |x| Some(x + change)).unwrap()
    } else {
        let old = a.load(std::sync::atomic::Ordering::SeqCst);
        a.fetch_add(change, std::sync::atomic::Ordering::SeqCst);
        old
    };
    assert_eq!(result, if pre_op { value + change } else { value });
    assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), value + change);
}

fn test_incdec_primitive<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq>(a: &mut T, change: T, pre_op: bool) {
    let original = *a;
    let result = if pre_op {
        *a = *a + change;
        *a
    } else {
        let old = *a;
        *a = *a + change;
        old
    };
    assert_eq!(result, if pre_op { original + change } else { original });
    assert_eq!(*a, original + change);
}

mod atomic {
    use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, Ordering};

    pub trait AtomicArithmetic {
        fn fetch_add(&self, val: Self, order: Ordering) -> Self;
        fn fetch_sub(&self, val: Self, order: Ordering) -> Self;
        fn load(&self, order: Ordering) -> Self;
        fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<Self, Self>
            where F: FnMut(Self) -> Option<Self>;
        fn new(val: Self) -> Self;
    }

    macro_rules! impl_atomic_arithmetic {
        ($type:ty, $atomic_type:ident) => {
            impl AtomicArithmetic for $atomic_type {
                fn fetch_add(&self, val: Self, order: Ordering) -> Self {
                    <$atomic_type>::fetch_add(&self, val, order)
                }
                fn fetch_sub(&self, val: Self, order: Ordering) -> Self {
                    <$atomic_type>::fetch_sub(&self, val, order)
                }
                fn load(&self, order: Ordering) -> Self {
                    <$atomic_type>::load(&self, order)
                }
                fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<Self, Self>
                where F: FnMut(Self) -> Option<Self> {
                    <$atomic_type>::fetch_update(&self, set_order, fetch_order, f)
                }
                fn new(val: Self) -> Self {
                    <$atomic_type>::new(val)
                }
            }
        };
    }

    impl_atomic_arithmetic!(bool, AtomicBool);
    impl_atomic_arithmetic!(i8, AtomicI8);
    impl_atomic_arithmetic!(u8, AtomicU8);
    impl_atomic_arithmetic!(i16, AtomicI16);
    impl_atomic_arithmetic!(u16, AtomicU16);
    impl_atomic_arithmetic!(i32, AtomicI32);
    impl_atomic_arithmetic!(u32, AtomicU32);
    impl_atomic_arithmetic!(i64, AtomicI64);
    impl_atomic_arithmetic!(u64, AtomicU64);
}