use std::process;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static mut A: std::sync::atomic::AtomicPtr<$type> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            static mut B: std::sync::atomic::AtomicPtr<$type> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

            unsafe {
                if A.load(std::sync::atomic::Ordering::SeqCst) != std::ptr::null_mut() {
                    process::exit(1);
                }
                if B.load(std::sync::atomic::Ordering::SeqCst) != &$value as *const $type as *mut $type {
                    process::exit(1);
                }
                if A.compare_exchange(
                    std::ptr::null_mut(),
                    &$value as *const $type as *mut $type,
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                ).is_err() {
                    process::exit(1);
                }
                if A.load(std::sync::atomic::Ordering::SeqCst) != &$value as *const $type as *mut $type {
                    process::exit(1);
                }
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        {
            test_simple_assign!(bool, $value);
            test_simple_assign!(i8, $value);
            test_simple_assign!(u8, $value);
            test_simple_assign!(i16, $value);
            test_simple_assign!(u16, $value);
            test_simple_assign!(i32, $value);
            test_simple_assign!(u32, $value);
            test_simple_assign!(i64, $value);
            test_simple_assign!(u64, $value);
            test_simple_assign!(f32, $value);
            test_simple_assign!(f64, $value);
        }
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);

    let i = 0;
    test_simple_assign!(*const i32, &i as *const i32);

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    unsafe {
        let s1 = std::sync::atomic::AtomicPtr::new(&init as *const S as *mut S);
        let s2 = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        copy = *s1.load(std::sync::atomic::Ordering::SeqCst);
        if init.a != copy.a {
            process::exit(1);
        }

        s2.store(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
        copy = *s2.load(std::sync::atomic::Ordering::SeqCst);
        if init.a != copy.a {
            process::exit(1);
        }

        copy = *s1.load(std::sync::atomic::Ordering::SeqCst);
        if init.a != copy.a {
            process::exit(1);
        }

        copy = *s2.load(std::sync::atomic::Ordering::SeqCst);
        if init.a != copy.a {
            process::exit(1);
        }
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}