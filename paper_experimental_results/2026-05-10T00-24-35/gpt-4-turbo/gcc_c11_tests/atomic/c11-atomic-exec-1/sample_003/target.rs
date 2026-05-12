use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};

fn test_simple_assign<T: PartialEq + Copy + Default>(value: T) {
    let mut a = Atomic::new(T::default());
    let b = Atomic::new(value);

    assert_eq!(a.load(Ordering::SeqCst), T::default());
    assert_eq!(b.load(Ordering::SeqCst), value);

    a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), value);
}

fn test_simple_assign_arith(value: i64) {
    test_simple_assign::<bool>(value != 0);
    test_simple_assign::<i8>(value as i8);
    test_simple_assign::<u8>(value as u8);
    test_simple_assign::<i16>(value as i16);
    test_simple_assign::<u16>(value as u16);
    test_simple_assign::<i32>(value as i32);
    test_simple_assign::<u32>(value as u32);
    test_simple_assign::<i64>(value);
    test_simple_assign::<u64>(value as u64);
    test_simple_assign::<isize>(value as isize);
    test_simple_assign::<usize>(value as usize);
}

fn test_simple_assign_floating(value: f64) {
    test_simple_assign::<f32>(value as f32);
    test_simple_assign::<f64>(value);
}

fn test_simple_assign_complex(value: f64, imag: f64) {
    test_simple_assign::<(f32, f32)>((value as f32, imag as f32));
    test_simple_assign::<(f64, f64)>((value, imag));
}

fn main() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1 << 63);
    test_simple_assign_floating(1.5);
    test_simple_assign_complex(2.5, 3.5);
    
    // Pointer and complex structure tests
    let i = 0;
    test_simple_assign::<*const i32>(std::ptr::null());
    test_simple_assign::<*const i32>(&i as *const i32);
    
    struct S {
        a: [i16; 1024],
    }
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let s1 = Atomic::new(init);
    let s2 = Atomic::new(S { a: [0; 1024] });
    let copy = s1.load(Ordering::SeqCst);
    assert_eq!(init.a, copy.a);
    s2.store(copy, Ordering::SeqCst);
    let copy2 = s2.load(Ordering::SeqCst);
    assert_eq!(copy.a, copy2.a);
}