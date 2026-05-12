fn main() {
    struct S {
        inner: {
            i: i32,
        },
        a: [i32; 4],
    }

    let layout = std::alloc::Layout::new::<S>()
        .extend(std::alloc::Layout::array::<i32>(4).unwrap());

    let p = unsafe { std::alloc::alloc(layout) as *mut S };

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).inner.i = 7;
        (*p).a[0] = 11;
        (*p).a[3] = 22;
    }

    unsafe {
        if (*p).inner.i != 7 || (*p).a[0] != 11 || (*p).a[3] != 22 {
            return 4;
        }
    }

    unsafe {
        std::alloc::dealloc(p as *mut u8, layout);
    }

    0
}