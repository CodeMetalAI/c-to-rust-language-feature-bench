fn main() {
    check_align::<bool>();
    check_align::<char>();
    check_align::<i8>();
    check_align::<u8>();
    check_align::<i16>();
    check_align::<u16>();
    check_align::<i32>();
    check_align::<u32>();
    check_align::<i64>();
    check_align::<u64>();
    check_align::<f32>();
    check_align::<f64>();
    check_align::<f64>();
    check_align::<complex::ComplexF32>();
    check_align::<complex::ComplexF64>();
    check_align::<complex::ComplexF64>();
    std::process::exit(0);
}

fn check_align<T>() {
    let x = [std::mem::zeroed::<T>(), 0];
    let size_of_x = std::mem::size_of::<[T; 1]>();
    if std::mem::align_of::<T>() > size_of_x {
        panic!("alignof(T) is greater than size of x");
    }
}