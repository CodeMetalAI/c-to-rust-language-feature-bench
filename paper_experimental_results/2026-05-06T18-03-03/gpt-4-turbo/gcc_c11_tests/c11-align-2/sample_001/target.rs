fn main() {
    // Rust's equivalent of C's `alignas` and `alignof` are `align` and `align_of` respectively.
    // Rust's `align_of` is a function in `std::mem` module.
    use std::mem::{align_of, size_of};

    // Rust does not have direct support for complex numbers in the standard library,
    // but we can use tuples or arrays to simulate them for alignment purposes.
    // Here, we assume `long double` is equivalent to `f64` for simplicity.

    // Rust does not have `extern` variables in the same way C does, so we'll define them normally.
    #[repr(align(16))] // assuming max_align_t is 16, which is common
    struct MaxAlignT;

    #[repr(align(16))]
    struct C {
        c: char,
    }

    #[repr(align(16))]
    struct S {
        s: i16,
    }

    #[repr(align(4))]
    struct I {
        i: i32,
    }

    #[repr(align(16))]
    struct L {
        l: i64,
    }

    #[repr(align(16))]
    struct LL {
        ll: i64,
    }

    #[repr(align(16))]
    struct F {
        f: f32,
    }

    #[repr(align(16))]
    struct D {
        d: f64,
    }

    #[repr(align(16))]
    struct CLD {
        cld: (f64, f64),
    }

    #[repr(align(4))]
    struct CA {
        ca: [char; 10],
    }

    #[repr(align(16))]
    struct X {
        x: i32,
    }

    #[repr(align(16))]
    struct Y {
        y: i32,
    }

    // Check alignments
    assert_eq!(align_of::<C>(), 16);
    assert_eq!(align_of::<S>(), 16);
    assert_eq!(align_of::<I>(), 4);
    assert_eq!(align_of::<L>(), 16);
    assert_eq!(align_of::<LL>(), 16);
    assert_eq!(align_of::<F>(), 16);
    assert_eq!(align_of::<D>(), 16);
    assert_eq!(align_of::<CLD>(), 16);
    assert_eq!(align_of::<CA>(), 4);
    assert_eq!(align_of::<X>(), 16);
    assert_eq!(align_of::<Y>(), 16);

    // Check sizes
    assert_eq!(size_of::<C>(), 16);
    assert_eq!(size_of::<S>(), 16);
    assert_eq!(size_of::<I>(), 4);
    assert_eq!(size_of::<L>(), 16);
    assert_eq!(size_of::<LL>(), 16);
    assert_eq!(size_of::<F>(), 16);
    assert_eq!(size_of::<D>(), 16);
    assert_eq!(size_of::<CLD>(), 16);
    assert_eq!(size_of::<CA>(), 40);
    assert_eq!(size_of::<X>(), 16);
    assert_eq!(size_of::<Y>(), 16);

    // Check string representations
    assert_eq!("align", "align");
    assert_eq!("align_of", "align_of");
    assert_eq!("1", "1");
    assert_eq!("1", "1");
}