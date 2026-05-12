fn main() {
    check_align::<u8>();
    check_align::<i8>();
    check_align::<u16>();
    check_align::<i16>();
    check_align::<u32>();
    check_align::<i32>();
    check_align::<u64>();
    check_align::<i64>();
    check_align::<f32>();
    check_align::<f64>();
    check_align::<f32i32>();
    check_align::<f64i32>();
    check_align::<f64>();

    std::process::exit(0);
}

fn check_align<T>() {
    struct Wrapper<T> {
        c: u8,
        v: T,
    }

    let wrapper = Wrapper { c: 0, v: T::default() };
    let alignment = std::mem::align_of::<T>();
    if alignment > std::mem::size_of::<Wrapper>().align_offset() + 1 {
        std::process::exit(1);
    }
}