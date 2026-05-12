fn main() {
    #[repr(C)]
    struct S {
        inner: std::mem::MaybeUninit<i32>,
        a: [i32],
    }

    let layout = std::mem::Layout::new::<S>();
    if layout.offset_of(0).unwrap() != 0 || layout.offset_of(1).unwrap() != std::mem::size_of::<S>() {
        return 2;
    }

    let mut p = Box::<[u8]>::leak(vec![0; layout.size()].into_boxed_slice());
    let p = p.as_mut_ptr() as *mut S;
    unsafe {
        (*p).inner.write(7);
        (*p).a.get_unchecked_mut(0).write(11);
        (*p).a.get_unchecked_mut(3).write(22);
    }

    unsafe {
        if (*p).inner.assume_init() != 7 || (*p).a.get_unchecked(0) != 11 || (*p).a.get_unchecked(3) != 22 {
            return 4;
        }
    }
}