fn main() {
    #[repr(C)]
    struct S {
        inner: Inner,
        a: [i32; 0],
    }

    #[repr(C)]
    struct Inner {
        i: i32,
    }

    if std::mem::offset_of!(S, inner)!= 0 {
        return 1;
    }

    if std::mem::size_of::<S>()!= std::mem::size_of::<Inner>() {
        return 2;
    }

    let p = Box::new([0; 4]);
    let p = p.as_mut_ptr() as *mut S;

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).inner.i = 7;
        (*p).a[0] = 11;
        (*p).a[3] = 22;

        if (*p).inner.i!= 7 || (*p).a[0]!= 11 || (*p).a[3]!= 22 {
            return 4;
        }
    }

    return 0;
}