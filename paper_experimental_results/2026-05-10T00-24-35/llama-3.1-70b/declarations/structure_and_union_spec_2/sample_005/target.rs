fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64],
    }

    #[repr(C)]
    struct Ss {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    if std::mem::offset_of!(S, d)!= std::mem::size_of::<S>() {
        std::process::exit(1);
    }

    let s1 = std::boxed::Box::new([0; std::mem::size_of::<S>() + 8 * std::mem::size_of::<f64>()]);
    let s2 = std::boxed::Box::new([0; std::mem::size_of::<S>() + 5 * std::mem::size_of::<f64>()]);

    let s1_ptr = s1.as_mut_ptr() as *mut S;
    let s2_ptr = s2.as_mut_ptr() as *mut S;

    unsafe {
        (*s1_ptr).d[0] = 42.0;
        (*s2_ptr).d[0] = 24.0;

        if (*s1_ptr).d[0]!= 42.0 || (*s2_ptr).d[0]!= 24.0 {
            std::process::exit(1);
        }
    }
}