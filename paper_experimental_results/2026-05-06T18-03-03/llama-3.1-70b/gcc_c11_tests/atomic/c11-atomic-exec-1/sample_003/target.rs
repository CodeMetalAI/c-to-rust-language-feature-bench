use std::mem;

#[derive(Debug, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    let values = [0, 1, 2, -1, 1 << 63, 1.5, 2.5 + 3.5i];
    let types = [
        bool,
        i8,
        u8,
        i16,
        u16,
        i32,
        u32,
        i64,
        u64,
        f32,
        f64,
        std::complex::Complex64,
        std::complex::Complex128,
        std::complex::Complex::<f64>,
    ];

    for value in &values {
        for ty in &types {
            let mut a: std::sync::atomic::Atomic<Ty> = std::sync::atomic::Atomic::new(0);
            let b: Ty = *value;
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b, *value);
            a.store(b, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), b);
        }
    }

    let mut i = 0;
    let values = [0 as *const i32, &mut i as *mut i32];
    for value in &values {
        let mut a: std::sync::atomic::Atomic<*const i32> = std::sync::atomic::Atomic::new(0);
        let b: *const i32 = *value;
        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0 as *const i32);
        assert_eq!(b, *value);
        a.store(b, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), b);
    }

    let mut init = S {
        a: [0; 1024],
    };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = std::sync::atomic::Atomic::new(init);
    let mut s2 = std::sync::atomic::Atomic::new(init);
    let copy = s1.swap(init);
    assert_eq!(init, copy);
    let copy = s2.swap(s1.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(init, copy);
    let copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    let copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign();
}