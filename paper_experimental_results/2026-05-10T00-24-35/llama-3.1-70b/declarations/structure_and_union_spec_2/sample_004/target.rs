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

    let s1 = unsafe { std::alloc::alloc(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 8) };
    let s2 = unsafe { std::alloc::alloc(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 5) };

    if s1.is_null() || s2.is_null() {
        return;
    }

    unsafe {
        let s1 = s1 as *mut S;
        let s2 = s2 as *mut S;

        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;

        if (*s1).d[0] != 42.0 || (*s2).d[0] != 24.0 {
            return;
        }

        std::alloc::dealloc(s1 as _, std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 8, std::alloc::Layout::from_size_align(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 8, 1).unwrap());
        std::alloc::dealloc(s2 as _, std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 5, std::alloc::Layout::from_size_align(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 5, 1).unwrap());
    }
}