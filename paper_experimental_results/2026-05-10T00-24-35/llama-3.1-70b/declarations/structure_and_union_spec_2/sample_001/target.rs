fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct Ss {
        n: i32,
    }

    assert!(std::mem::size_of::<S>() >= std::mem::size_of::<Ss>());
    assert_eq!(std::mem::offset_of!(S, d), std::mem::size_of::<S>());

    let s1: Box<[f64]> = vec![0.0; 8].into_boxed_slice();
    let s2: Box<[f64]> = vec![0.0; 5].into_boxed_slice();

    assert!(!s1.is_empty() &&!s2.is_empty());

    let s1_ptr = s1.as_mut_ptr() as *mut f64;
    let s2_ptr = s2.as_mut_ptr() as *mut f64;

    unsafe {
        *s1_ptr = 42.0;
        *s2_ptr = 24.0;

        assert_eq!(*s1_ptr, 42.0);
        assert_eq!(*s2_ptr, 24.0);
    }

    drop(s1);
    drop(s2);
}