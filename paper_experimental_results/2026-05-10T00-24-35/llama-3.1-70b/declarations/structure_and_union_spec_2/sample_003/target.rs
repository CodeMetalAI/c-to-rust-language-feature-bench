fn main() {
    struct S {
        n: i32,
        _d: [u8; 0], // C-style flexible array member
    };

    struct SS {
        n: i32,
    };

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        return;
    }

    if std::mem::offset_of!(S, _d) != std::mem::size_of::<S>() {
        return;
    }

    let s1: Box<[i32; 8]> = vec![0; 8].into_boxed_slice();
    let s2: Box<[i32; 5]> = vec![0; 5].into_boxed_slice();

    if s1.is_empty() || s2.is_empty() {
        return;
    }

    unsafe {
        *(s1.as_mut_ptr() as *mut f64) = 42.0;
        *(s2.as_mut_ptr() as *mut f64) = 24.0;
    }

    if unsafe { *(s1.as_ptr() as *const f64) } != 42.0
        || unsafe { *(s2.as_ptr() as *const f64) } != 24.0
    {
        return;
    }
}