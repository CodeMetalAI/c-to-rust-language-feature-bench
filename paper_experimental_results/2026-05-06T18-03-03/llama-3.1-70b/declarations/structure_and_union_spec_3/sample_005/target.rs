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

    let p = Box::new([S {
        inner: Inner { i: 7 },
        a: [],
    }]);

    let p = p.as_mut_ptr() as *mut S;

    if std::ptr::addr_of!((*p).inner.i)!= p as *const _ as *const i32 {
        std::process::exit(1);
    }

    if std::ptr::addr_of!((*p).a)!= (p as *const _ as usize + std::mem::size_of::<S>()) as *const i32 {
        std::process::exit(2);
    }

    let p = unsafe { Box::from_raw(p) };

    if p.inner.i!= 7 || p.a[0]!= 0 || p.a[3]!= 0 {
        std::process::exit(4);
    }

    std::mem::drop(p);
    std::process::exit(0);
}