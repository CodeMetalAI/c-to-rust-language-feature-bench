fn check_align<T>() {
    struct S<T> {
        c: u8,
        v: T,
    };
    if ::std::mem::size_of::<T>() > ::std::mem::size_of::<S<T>>();
    panic!("_Alignof failed");
}

fn main() {
    check_align::<bool>();
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
    check_align::<i128>();
    check_align::<u128>();
    std::process::exit(0);
}