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

    let s1 = Box::new([0u8; std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 8]);
    let s2 = Box::new([0u8; std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 5]);

    let s1 = s1.as_mut_ptr() as *mut S;
    let s2 = s2.as_mut_ptr() as *mut S;

    unsafe {
        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;

        if (*s1).d[0]!= 42.0 || (*s2).d[0]!= 24.0 {
            return;
        }
    }
}