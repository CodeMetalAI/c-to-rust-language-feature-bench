fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        return;
    }

    if std::mem::offset_of!(S, d) != std::mem::size_of::<S>() {
        return;
    }

    let s1 = std::boxed::Box::<[f64]>::leak(vec![0.0; 8].into_boxed_slice());
    let s2 = std::boxed::Box::<[f64]>::leak(vec![0.0; 5].into_boxed_slice());

    let s1_ptr = s1.as_mut_ptr() as *mut S;
    let s2_ptr = s2.as_mut_ptr() as *mut S;

    unsafe {
        (*s1_ptr).d[0] = 42.0;
        (*s2_ptr).d[0] = 24.0;

        if (*s1_ptr).d[0] != 42.0 || (*s2_ptr).d[0] != 24.0 {
            return;
        }
    }
}