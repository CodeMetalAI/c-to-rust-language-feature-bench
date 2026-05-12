use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;
use std::mem;
use std::ptr;

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($atomic_type:ty, $typ:ty, $value:expr) => {
            {
                static A: AtomicUsize = AtomicUsize::new(0);
                static B: $atomic_type = AtomicUsize::new($value as usize);

                if A.load(Ordering::SeqCst) != 0 {
                    panic!();
                }
                if B.load(Ordering::SeqCst) != $value as usize {
                    panic!();
                }

                A.store(B.load(Ordering::SeqCst), Ordering::SeqCst);

                if A.load(Ordering::SeqCst) != $value as usize {
                    panic!();
                }
            }
        };
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {
            test_simple_assign!(AtomicUsize, usize, $value);
        };
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1i32 as usize);
    test_simple_assign_arith!((1u64 << 63) as usize);
    // Note: Atomic floats/complex numbers are not directly supported in Rust.
    //       These would require further handling with possibly other crates.

    // Handling for pointers
    {
        static mut I: i32 = 0;
        let ptr_null: *mut i32 = ptr::null_mut();
        let ptr_i: *mut i32 = unsafe { &mut I };

        let a = Arc::new(AtomicPtr::new(ptr_null));
        let b = Arc::new(AtomicPtr::new(ptr_i));

        if a.load(Ordering::SeqCst) != ptr::null_mut() {
            panic!();
        }
        if b.load(Ordering::SeqCst) != ptr_i {
            panic!();
        }

        a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);

        if a.load(Ordering::SeqCst) != ptr_i {
            panic!();
        }
    }

    // Handling for structs
    #[derive(Clone, Copy, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = Arc::new(init);
    let s2 = Arc::new(AtomicPtr::new(ptr::null_mut()));

    unsafe {
        let raw_s1 = Arc::into_raw(Arc::clone(&s1)) as *mut S;
        s2.store(raw_s1, Ordering::SeqCst);
    }

    copy = *s1;
    if init != copy {
        panic!();
    }

    let s2_copy_ptr = s2.load(Ordering::SeqCst) as *const S;
    copy = unsafe { *s2_copy_ptr };
    if init != copy {
        panic!();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}