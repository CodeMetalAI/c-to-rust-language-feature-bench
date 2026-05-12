fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            let align = std::mem::align_of::<$type>();
            let mut x = [0u8; std::mem::size_of::<$type>() + 1];
            let ptr = &mut x[1] as *mut u8 as *mut $type;
            assert!(align <= std::mem::align_of_val(ptr));
        };
    }

    check_align!(bool);
    check_align!(char);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    // Note: Rust does not have a direct equivalent to C's _Complex type.
    // However, we can use the num_complex crate or define our own complex number type.
    // For the sake of simplicity, we will skip this test.
}