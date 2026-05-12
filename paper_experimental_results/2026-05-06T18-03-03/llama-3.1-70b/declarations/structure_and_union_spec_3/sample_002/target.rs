fn main() {
    #[repr(C)]
    struct S {
        i: i32,
        a: [i32; 0],
    }

    if std::mem::offset_of!(S, i) != 0 {
        return;
    }

    if std::mem::size_of::<S>() != std::mem::size_of::<i32>() {
        return;
    }

    let mut p = Box::new([0; std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>()]);
    let p = p.as_mut_ptr() as *mut S;

    unsafe {
        (*p).i = 7;
        (*p).a[0] = 11;
        (*p).a[3] = 22;

        if (*p).i != 7 || (*p).a[0] != 11 || (*p).a[3] != 22 {
            return;
        }
    }
}