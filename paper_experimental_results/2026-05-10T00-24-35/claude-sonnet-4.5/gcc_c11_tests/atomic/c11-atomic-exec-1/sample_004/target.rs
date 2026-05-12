use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::cell::Cell;

fn abort() -> ! {
    std::process::exit(1);
}

macro_rules! TEST_SIMPLE_ASSIGN_ATOMIC {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        thread_local! {
            static A: $atomic_type = <$atomic_type>::new(<$value_type>::default());
            static B: $atomic_type = <$atomic_type>::new($value as $value_type);
        }
        
        A.with(|a| {
            if a.load(Ordering::SeqCst) != <$value_type>::default() {
                abort();
            }
        });
        
        B.with(|b| {
            if b.load(Ordering::SeqCst) != ($value as $value_type) {
                abort();
            }
        });
        
        let val = B.with(|b| b.load(Ordering::SeqCst));
        A.with(|a| a.store(val, Ordering::SeqCst));
        
        if val != ($value as $value_type) {
            abort();
        }
        
        A.with(|a| {
            if a.load(Ordering::SeqCst) != ($value as $value_type) {
                abort();
            }
        });
    }};
}

macro_rules! TEST_SIMPLE_ASSIGN_CELL {
    ($value_type:ty, $value:expr) => {{
        thread_local! {
            static A: Cell<$value_type> = Cell::new(<$value_type>::default());
            static B: Cell<$value_type> = Cell::new($value as $value_type);
        }
        
        A.with(|a| {
            if a.get() != <$value_type>::default() {
                abort();
            }
        });
        
        B.with(|b| {
            if b.get() != ($value as $value_type) {
                abort();
            }
        });
        
        let val = B.with(|b| b.get());
        A.with(|a| a.set(val));
        
        if val != ($value as $value_type) {
            abort();
        }
        
        A.with(|a| {
            if a.get() != ($value as $value_type) {
                abort();
            }
        });
    }};
}

macro_rules! TEST_SIMPLE_ASSIGN_ARITH {
    ($value:expr) => {{
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicBool, bool, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI8, i8, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI8, i8, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicU8, u8, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI16, i16, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicU16, u16, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI32, i32, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicU32, u32, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI64, i64, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicU64, u64, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicI64, i64, $value);
        TEST_SIMPLE_ASSIGN_ATOMIC!(AtomicU64, u64, $value);
        TEST_SIMPLE_ASSIGN_CELL!(f32, $value);
        TEST_SIMPLE_ASSIGN_CELL!(f64, $value);
        TEST_SIMPLE_ASSIGN_CELL!(f64, $value);
        TEST_SIMPLE_ASSIGN_CELL!((f32, f32), $value);
        TEST_SIMPLE_ASSIGN_CELL!((f64, f64), $value);
        TEST_SIMPLE_ASSIGN_CELL!((f64, f64), $value);
    }};
}

fn test_simple_assign() {
    TEST_SIMPLE_ASSIGN_ARITH!(0);
    TEST_SIMPLE_ASSIGN_ARITH!(1);
    TEST_SIMPLE_ASSIGN_ARITH!(2);
    TEST_SIMPLE_ASSIGN_ARITH!(-1);
    TEST_SIMPLE_ASSIGN_ARITH!(1u64 << 63);
    TEST_SIMPLE_ASSIGN_ARITH!(1.5);
    TEST_SIMPLE_ASSIGN_ARITH!((2.5, 3.5));
    
    thread_local! {
        static I: Cell<i32> = Cell::new(0);
    }
    
    TEST_SIMPLE_ASSIGN_CELL!(usize, 0);
    I.with(|i_ref| {
        let ptr = i_ref.as_ptr() as usize;
        TEST_SIMPLE_ASSIGN_CELL!(usize, ptr);
    });
    
    #[derive(Clone, Copy, PartialEq)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    thread_local! {
        static S1: Cell<S> = Cell::new(S { a: [0; 1024] });
        static S2: Cell<S> = Cell::new(S { a: [0; 1024] });
    }
    
    S1.with(|s1| s1.set(init));
    let copy = S1.with(|s1| s1.get());
    if init != copy {
        abort();
    }
    
    S2.with(|s2| s2.set(S1.with(|s1| s1.get())));
    let copy = S2.with(|s2| s2.get());
    if init != copy {
        abort();
    }
    
    let copy = S1.with(|s1| s1.get());
    if init != copy {
        abort();
    }
    
    let copy = S2.with(|s2| s2.get());
    if init != copy {
        abort();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}