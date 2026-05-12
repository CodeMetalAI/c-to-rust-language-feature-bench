use std::mem;

fn main() {
    // Test alignment support
    let _c: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _s: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _i: align_to::<{ mem::align_of::<i32>() }> = align_to::new_uninit().assume_init();
    let _l: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _ll: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _f: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _d: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _cld: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    let _ca: align_to::<{ mem::align_of::<u8>() }> = align_to::new_uninit().assume_init();
    let _x: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() + 0 }> = align_to::new_uninit().assume_init();
    let _y: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();

    // Test alignment of function-local variable
    fn func() {
        let _auto_ll: align_to::<{ mem::align_of::<mem::MaybeUninit<max_align_t>>() }> = align_to::new_uninit().assume_init();
    }
    func();

    // Test alignment of struct (valid, but useless)
    struct S;
    let _s: align_to::<0> = align_to::new_uninit().assume_init();

    // Test alignment macros
    assert_eq!(stringify!(align_to), "align_to");
    assert_eq!(stringify!(mem::align_of), "mem::align_of");
    assert_eq!(stringify!(1), "1");
    assert_eq!(stringify!(1), "1");
}

// Helper type for alignment
mod align_to {
    use std::mem;

    pub struct AlignTo<T, const N: usize>(T, [u8; N]);

    pub fn new_uninit<T, const N: usize>() -> mem::MaybeUninit<AlignTo<T, N>> {
        mem::MaybeUninit::uninit()
    }

    impl<T, const N: usize> AlignTo<T, N> {
        pub fn assume_init(self: mem::MaybeUninit<Self>) -> Self {
            unsafe { self.assume_init() }
        }
    }
}

// Helper type for max_align_t
#[repr(align(16))]
pub struct max_align_t;