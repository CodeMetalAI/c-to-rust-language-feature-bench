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

    if std::mem::offset_of!(S, d)!= std::mem::size_of::<S>() {
        return;
    }

    let s1 = vec![0.0; 8];
    let s2 = vec![0.0; 5];

    let s1_ptr = s1.as_mut_ptr();
    let s2_ptr = s2.as_mut_ptr();

    unsafe {
        *s1_ptr.offset(0) = 42.0;
        *s2_ptr.offset(0) = 24.0;

        if *s1_ptr.offset(0)!= 42.0 || *s2_ptr.offset(0)!= 24.0 {
            return;
        }
    }
}