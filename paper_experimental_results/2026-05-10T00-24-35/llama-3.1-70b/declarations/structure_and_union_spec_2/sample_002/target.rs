fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    assert!(std::mem::size_of::<S>() >= std::mem::size_of::<SS>());

    assert_eq!(std::mem::offset_of!(S, d), std::mem::size_of::<S>());

    let s1 = Box::leak(Box::new([0.0; 8]) as Box<[f64]>);
    let s2 = Box::leak(Box::new([0.0; 5]) as Box<[f64]>);

    let s1_ptr = s1.as_mut_ptr();
    let s2_ptr = s2.as_mut_ptr();

    unsafe {
        *(s1_ptr as *mut [f64; 8]).get_unchecked_mut(0) = 42.0;
        *(s2_ptr as *mut [f64; 5]).get_unchecked_mut(0) = 24.0;
    }

    assert_eq!(unsafe { *(s1_ptr as *mut f64) }, 42.0);
    assert_eq!(unsafe { *(s2_ptr as *mut f64) }, 24.0);
}