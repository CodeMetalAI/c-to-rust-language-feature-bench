fn main() {
    test_simple_assign();
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith((2.5, 3.5));
    let mut i: i32 = 0;
    test_simple_assign_arith(&mut i as *mut i32);
    test_simple_assign_arith(&i as *const i32);

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct S([i16; 1024]);

    let mut init = S([0; 1024]);
    for (i, e) in init.0.iter_mut().enumerate() {
        *e = i as i16;
    }

    let s1 = init.clone();
    let s2 = s1.clone();
    assert_eq!(init, s2);
    let s3 = s1.clone();
    assert_eq!(init, s3);
    let s4 = s2.clone();
    assert_eq!(init, s4);
}

fn test_simple_assign_arith<T: Copy + PartialEq + std::fmt::Debug>(value: T) {
    let mut a = std::sync::atomic::AtomicCell::new(value);
    let b = std::sync::atomic::AtomicCell::new(value);
    assert_eq!(a.into_inner(), value);
    assert_eq!(b.into_inner(), value);
    a.store(b.into_inner());
    assert_eq!(a.into_inner(), value);
}

fn test_simple_assign<T: Copy + PartialEq + std::fmt::Debug>(value: T) {
    let mut a = std::sync::atomic::AtomicCell::new(value);
    let b = std::sync::atomic::AtomicCell::new(value);
    assert_eq!(a.into_inner(), value);
    assert_eq!(b.into_inner(), value);
    a.store(b.into_inner());
    assert_eq!(a.into_inner(), value);
}