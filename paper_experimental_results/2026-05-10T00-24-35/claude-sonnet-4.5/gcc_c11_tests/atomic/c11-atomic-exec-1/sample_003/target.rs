use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::cell::Cell;

fn abort() -> ! {
    std::process::exit(1);
}

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        static A: $atomic_type = <$atomic_type>::new(0 as $value_type);
        static B: $atomic_type = <$atomic_type>::new($value as $value_type);
        
        if A.load(Ordering::SeqCst) != 0 as $value_type {
            abort();
        }
        if B.load(Ordering::SeqCst) != ($value as $value_type) {
            abort();
        }
        let b_val = B.load(Ordering::SeqCst);
        A.store(b_val, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != ($value as $value_type) {
            abort();
        }
    }};
}

macro_rules! test_simple_assign_cell {
    ($value_type:ty, $value:expr) => {{
        thread_local! {
            static A: Cell<$value_type> = Cell::new(0.0 as $value_type);
            static B: Cell<$value_type> = Cell::new($value as $value_type);
        }
        
        A.with(|a| {
            if a.get() != 0.0 as $value_type {
                abort();
            }
        });
        B.with(|b| {
            if b.get() != ($value as $value_type) {
                abort();
            }
            A.with(|a| {
                a.set(b.get());
                if a.get() != ($value as $value_type) {
                    abort();
                }
            });
        });
    }};
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {{
        test_simple_assign_atomic!(AtomicBool, bool, $value);
        test_simple_assign_atomic!(AtomicI8, i8, $value);
        test_simple_assign_atomic!(AtomicI8, i8, $value);
        test_simple_assign_atomic!(AtomicU8, u8, $value);
        test_simple_assign_atomic!(AtomicI16, i16, $value);
        test_simple_assign_atomic!(AtomicU16, u16, $value);
        test_simple_assign_atomic!(AtomicI32, i32, $value);
        test_simple_assign_atomic!(AtomicU32, u32, $value);
        test_simple_assign_atomic!(AtomicIsize, isize, $value);
        test_simple_assign_atomic!(AtomicUsize, usize, $value);
        test_simple_assign_atomic!(AtomicI64, i64, $value);
        test_simple_assign_atomic!(AtomicU64, u64, $value);
        test_simple_assign_cell!(f32, $value);
        test_simple_assign_cell!(f64, $value);
        test_simple_assign_cell!(f64, $value);
        test_simple_assign_cell!(f32, $value);
        test_simple_assign_cell!(f64, $value);
        test_simple_assign_cell!(f64, $value);
    }};
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    test_simple_assign_arith!(2.5);
    
    test_simple_assign_atomic!(AtomicUsize, usize, 0);
    static I: AtomicI32 = AtomicI32::new(0);
    let i_addr = &I as *const _ as usize;
    test_simple_assign_atomic!(AtomicUsize, usize, i_addr);
    
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